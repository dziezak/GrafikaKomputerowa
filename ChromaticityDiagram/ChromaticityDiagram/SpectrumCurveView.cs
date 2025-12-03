

using System;
using System.Collections.Generic;
using System.Linq;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Shapes;
using System.Windows.Controls;
using System.Windows;

namespace ChromaticityDiagram
{
    public class SpectrumCurveView
    {
        private readonly Canvas canvas;
        private readonly List<Point> controlPoints = new();

        public double LambdaMin { get; set; } = 380.0;
        public double LambdaMax { get; set; } = 780.0;

        public event EventHandler<Func<double, double>>? ChromaticityRequested;

        public Brush CurveStroke { get; set; } = Brushes.Blue;
        public double CurveThickness { get; set; } = 2.0;

        public bool ShowControlPoints { get; set; } = true;

        

        private Ellipse? draggedDot;
        private int draggedIndex = -1;
        private Point dragStartCanvasPos;

        private Ellipse CreateDraggableDot(Point p)
        {
            var dot = new Ellipse
            {
                Width = 8,
                Height = 8,
                Fill = Brushes.BlueViolet,
                Stroke = Brushes.White,
                StrokeThickness = 0.75,
                Cursor = Cursors.Hand,
                Tag = p
            };
            Canvas.SetLeft(dot, p.X - dot.Width / 2);
            Canvas.SetTop(dot,  p.Y - dot.Height / 2);

            dot.MouseLeftButtonDown += Dot_MouseLeftButtonDown;
            dot.MouseMove += Dot_MouseMove;
            dot.MouseLeftButtonUp += Dot_MouseLeftButtonUp;

            return dot;
        }

        private void Dot_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
        {
            draggedDot = sender as Ellipse;
            if (draggedDot == null) return;

            draggedDot.CaptureMouse();
            dragStartCanvasPos = e.GetPosition(canvas);

            double x = Canvas.GetLeft(draggedDot) + draggedDot.Width / 2;
            double y = Canvas.GetTop (draggedDot) + draggedDot.Height / 2;

            draggedIndex = controlPoints
                .Select((pt, idx) => (idx, dist: (pt.X - x) * (pt.X - x) + (pt.Y - y) * (pt.Y - y)))
                .OrderBy(t => t.dist).First().idx;

            e.Handled = true;
        }

        private void Dot_MouseMove(object sender, MouseEventArgs e)
        {
            if (draggedDot == null || e.LeftButton != MouseButtonState.Pressed) return;

            var pos = e.GetPosition(canvas);
            double dx = pos.X - dragStartCanvasPos.X;
            double dy = pos.Y - dragStartCanvasPos.Y;
            dragStartCanvasPos = pos;

            Canvas.SetLeft(draggedDot, Canvas.GetLeft(draggedDot) + dx);
            Canvas.SetTop (draggedDot, Canvas.GetTop (draggedDot) + dy);

            if (draggedIndex >= 0 && draggedIndex < controlPoints.Count)
            {
                double cx = Canvas.GetLeft(draggedDot) + draggedDot.Width / 2;
                double cy = Canvas.GetTop (draggedDot) + draggedDot.Height / 2;
                controlPoints[draggedIndex] = new Point(cx, cy);
            }

            Redraw();
            RaiseChromaticityRequested();
        }

        private void Dot_MouseLeftButtonUp(object sender, MouseButtonEventArgs e)
        {
            draggedDot?.ReleaseMouseCapture();
            draggedDot = null;
            draggedIndex = -1;
            e.Handled = true;
        }


        public SpectrumCurveView(Canvas hostCanvas)
        {
            canvas = hostCanvas ?? throw new ArgumentNullException(nameof(hostCanvas));
            canvas.MouseLeftButtonDown += OnMouseLeftButtonDown;
            canvas.SizeChanged += (_, __) => Redraw();
            RedrawAxes();
        }

        public void AddPoint(Point p)
        {
            controlPoints.Add(p);
            Redraw();
            RaiseChromaticityRequested();
        }

        public void Clear()
        {
            controlPoints.Clear();
            canvas.Children.Clear();
            RedrawAxes();
            RaiseChromaticityRequested();
        }

        private void OnMouseLeftButtonDown(object sender, MouseButtonEventArgs e)
        {
            var pos = e.GetPosition(canvas);
            controlPoints.Add(pos);
            Redraw();
            RaiseChromaticityRequested();
        }

        private void RedrawAxes()
        {
            double w = canvas.ActualWidth, h = canvas.ActualHeight;
            if (w <= 0 || h <= 0) return;

            var rect = new Rectangle
            {
                Width = w - 2,
                Height = h - 2,
                Stroke = Brushes.DarkGray,
                StrokeThickness = 1
            };
            Canvas.SetLeft(rect, 1);
            Canvas.SetTop(rect, 1);
            canvas.Children.Add(rect);
        }


        private void Redraw()
        {
            canvas.Children.Clear();
            RedrawAxes();

            if (controlPoints.Count == 0) return;

            var pts = controlPoints.OrderBy(p => p.X).ToList();

            Path path;

            if (pts.Count >= 4)
            {
                var fig = new PathFigure { StartPoint = pts[0], IsFilled = false, IsClosed = false };

                var segmentsPoints = new List<Point>();

                int remainder = (pts.Count - 1) % 3;
                int needed = (remainder == 0) ? 0 : (3 - remainder);
                var padded = pts.GetRange(1, pts.Count - 1).ToList();
                for (int i = 0; i < needed; i++)
                    padded.Add(pts[^1]);

                segmentsPoints.AddRange(padded);

                var polyBezier = new PolyBezierSegment(segmentsPoints, true);
                fig.Segments.Add(polyBezier);

                var geo = new PathGeometry();
                geo.Figures.Add(fig);

                path = new Path
                {
                    Stroke = CurveStroke,
                    StrokeThickness = CurveThickness,
                    Data = geo
                };
            }
            else
            {
                var poly = new Polyline
                {
                    Stroke = CurveStroke,
                    StrokeThickness = CurveThickness
                };
                foreach (var p in pts) poly.Points.Add(p);
                path = new Path { Stroke = Brushes.Transparent };
                canvas.Children.Add(poly);
            }

            canvas.Children.Add(path);

            if (ShowControlPoints)
            {
                foreach (var p in pts)
                {
                    var dot = CreateDraggableDot(p);
                    canvas.Children.Add(dot);
                }
            }
        }



        public Func<double, double> GetIntensityFunction()
        {
            double width = Math.Max(1, canvas.ActualWidth);
            double height = Math.Max(1, canvas.ActualHeight);

            var pts = controlPoints.OrderBy(p => p.X).ToList();
            if (pts.Count == 0)
                return _ => 0.0;

            double YToIntensity(double y) => Math.Clamp(1.0 - (y / height), 0.0, 1.0);
            double LambdaToX(double lambda) => (lambda - LambdaMin) / (LambdaMax - LambdaMin) * width;

            return lambda =>
            {
                double x = LambdaToX(lambda);

                var rightIdx = pts.FindIndex(p => p.X >= x);
                if (rightIdx <= 0) return YToIntensity(pts[0].Y);
                if (rightIdx >= pts.Count) return YToIntensity(pts[^1].Y);

                var pL = pts[rightIdx - 1];
                var pR = pts[rightIdx];

                double t = (x - pL.X) / (pR.X - pL.X);
                double y = pL.Y + t * (pR.Y - pL.Y);
                return YToIntensity(y);
            };
        }


        private void RaiseChromaticityRequested()
        {
            var func = GetIntensityFunction();
            ChromaticityRequested?.Invoke(this, func);
        }
    }
}
