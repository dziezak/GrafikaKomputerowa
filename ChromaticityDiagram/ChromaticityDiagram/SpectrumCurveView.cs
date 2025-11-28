

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

            // Rama
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

            // Podpisy osi możesz dodać jako TextBlock jeśli chcesz
            // (λ [nm] poziomo, Intensywność 0..1 pionowo).
        }

        private void Redraw()
        {
            canvas.Children.Clear();
            RedrawAxes();

            if (controlPoints.Count == 0) return;

            // Krzywa (na start Polyline)
            var poly = new Polyline
            {
                Stroke = CurveStroke,
                StrokeThickness = CurveThickness
            };
            foreach (var p in controlPoints.OrderBy(p => p.X))
                poly.Points.Add(p);
            canvas.Children.Add(poly);

            if (ShowControlPoints)
            {
                foreach (var p in controlPoints)
                {
                    var dot = new Ellipse
                    {
                        Width = 6,
                        Height = 6,
                        Fill = Brushes.BlueViolet,
                        Stroke = Brushes.White,
                        StrokeThickness = 0.5
                    };
                    Canvas.SetLeft(dot, p.X - dot.Width / 2);
                    Canvas.SetTop(dot, p.Y - dot.Height / 2);
                    canvas.Children.Add(dot);
                }
            }
        }

        public Func<double, double> GetIntensityFunction()
        {
            var pts = controlPoints.OrderBy(p => p.X).ToArray();
            double width = Math.Max(1, canvas.ActualWidth);
            double height = Math.Max(1, canvas.ActualHeight);

            double LambdaToX(double lambda)
                => (lambda - LambdaMin) / (LambdaMax - LambdaMin) * width;

            double YToIntensity(double y) => Math.Clamp(1.0 - (y / height), 0.0, 1.0);

            if (pts.Length == 0) return _ => 0.0;

            return lambda =>
            {
                double x = LambdaToX(lambda);

                var rightIdx = Array.FindIndex(pts, p => p.X >= x);
                if (rightIdx <= 0) return YToIntensity(pts[0].Y);
                if (rightIdx >= pts.Length) return YToIntensity(pts[^1].Y);

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
