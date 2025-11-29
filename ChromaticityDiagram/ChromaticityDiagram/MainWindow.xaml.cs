using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace ChromaticityDiagram;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    private Spectrum spectrum;
    private SpectrumCurveView spectrumCurveView;
    private Ellipse marker = new Ellipse { Width = 8, Height = 8, Fill = Brushes.Black, Stroke = Brushes.White, StrokeThickness = 1 };
    private Rectangle colorPatch = new Rectangle {Width = 80, Height = 40, Stroke = Brushes.Black, StrokeThickness = 1};
    
    public MainWindow()
    {
        InitializeComponent();
        Loaded += OnLoad;
        myCanvas.SizeChanged += (_, __) => DrawChromaticityStuff();
    }

    private void OnLoad(object sender, RoutedEventArgs e)
    {
        spectrum = Spectrum.LoadFromFile("Data/wykres.txt");
        spectrumCurveView = new SpectrumCurveView(spectrumCanvas);
        spectrumCurveView.ChromaticityRequested += (_, intensityFunc) =>
        {
            Console.WriteLine("Event fired!");
            UpdateChromaticityFromCurve(intensityFunc);
        };

        DrawChromaticityStuff();
    }
    
    
    private void ApplyPointsButton_Click(object sender, RoutedEventArgs e)
    {
        // Wczytaj punkty z textboxów
        var pts = new List<Point>();
        double Parse(string s) => double.TryParse(s, out var v) ? v : 0.0;

        pts.Add(new Point(Parse(P1X.Text), Parse(P1Y.Text)));
        pts.Add(new Point(Parse(P2X.Text), Parse(P2Y.Text)));
        pts.Add(new Point(Parse(P3X.Text), Parse(P3Y.Text)));
        pts.Add(new Point(Parse(P4X.Text), Parse(P4Y.Text)));
        pts.Add(new Point(Parse(P5X.Text), Parse(P5Y.Text)));

        // Przekaż do widoku krzywej
        spectrumCurveView.Clear();
        foreach (var p in pts) spectrumCurveView.AddPoint(p);
    }


    private void DrawChromaticityStuff()
    {
        if (myCanvas.ActualWidth <= 0 || myCanvas.ActualHeight <= 0 || spectrum == null) return;

        myCanvas.Children.Clear();
        DrawSpectrumCanvas(myCanvas, spectrum);  // brzeg podkowy (kolorowy)
        DrawSRGBGamut(myCanvas);                 // trójkąt gamutu sRGB

        // kolorowy prostokąt (początkowo neutralny)
        if (!myCanvas.Children.Contains(colorPatch))
            myCanvas.Children.Add(colorPatch);
        Canvas.SetLeft(colorPatch, 10);
        Canvas.SetTop (colorPatch, 10);
    }


    private Color WavelengthToRGB(double lambda)
    {
        double R = 0, G = 0, B = 0;
        if (lambda >= 380 && lambda < 440)
        {
            R = -(lambda - 440) / (440 - 380);
            G = 0;
            B = 1;
        }
        else if (lambda >= 440 && lambda < 490)
        {
            R = 0;
            G = (lambda - 440) / (490 - 440);
            B = 1;
        }
        else if (lambda >= 490 && lambda < 510)
        {
            R = 0;
            G = 1;
            B = -(lambda - 510) / (510 - 490);
        }
        else if (lambda >= 510 && lambda < 580)
        {
            R = (lambda - 510) / (580 - 510);
            G = 1;
            B = 0;
        }
        else if (lambda >= 580 && lambda < 645)
        {
            R = 1;
            G = -(lambda - 645) / (645 - 580);
            B = 0;
        }
        else if (lambda >= 645 && lambda <= 780)
        {
            R = 1;
            G = 0;
            B = 0;
        }
        return Color.FromRgb((byte)(R * 255), (byte)(G * 255), (byte)(B * 255));
    }



    public void DrawSpectrumCanvas(Canvas canvas, Spectrum spectrum)
    {
        double w = canvas.ActualWidth;
        double h = canvas.ActualHeight;

        if (w <= 0 || h <= 0 || spectrum == null || spectrum.Points.Count == 0)
            return;

        double prevX = double.NaN, prevY = double.NaN;
        double prevLambda = double.NaN;

        foreach (var s in spectrum.Points)
        {
            double sum = s.X + s.Y + s.Z;
            if (sum <= 0) continue;

            double x = s.X / sum;
            double y = s.Y / sum;

            if (double.IsNaN(x) || double.IsNaN(y)) continue;

            double px = x * w;
            double py = (1 - y) * h;

            if (double.IsNaN(px) || double.IsNaN(py)) continue;

            if (!double.IsNaN(prevX) && !double.IsNaN(prevY))
            {
                var brush = new SolidColorBrush(WavelengthToRGB(prevLambda));
                var line = new Line
                {
                    X1 = prevX,
                    Y1 = prevY,
                    X2 = px,
                    Y2 = py,
                    Stroke = brush,
                    StrokeThickness = 2
                };
                canvas.Children.Add(line);
            }

            prevX = px;
            prevY = py;
            prevLambda = s.Lambda;
        }
    }

    
    private void DrawSRGBGamut(Canvas canvas)
    {
        var w = canvas.ActualWidth; var h = canvas.ActualHeight;
        var rgb = new[] {
            (x:0.640, y:0.330),
            (x:0.300, y:0.600),
            (x:0.150, y:0.060)
        };

        var polygon = new Polygon
        {
            Stroke = Brushes.Gray,
            StrokeThickness = 1.5,
            Fill = new SolidColorBrush(Color.FromArgb(30, 128,128,128))
        };
        foreach (var p in rgb)
            polygon.Points.Add(new Point(p.x * w, (1 - p.y) * h));

        canvas.Children.Add(polygon);

        // Whitepoint D65
        var wp = (x:0.3127, y:0.3290);
        var wpMarker = new Ellipse { Width=6, Height=6, Fill=Brushes.White, Stroke=Brushes.Black, StrokeThickness=1 };
        Canvas.SetLeft(wpMarker, wp.x * w - 3);
        Canvas.SetTop (wpMarker, (1-wp.y) * h - 3);
        canvas.Children.Add(wpMarker);
    }

   
    private void SetBackgroundFromWeb(string url)
    {
        var bi = new BitmapImage();
        bi.BeginInit();
        bi.UriSource = new Uri(url, UriKind.Absolute);
        bi.CacheOption = BitmapCacheOption.OnLoad;
        bi.EndInit();

        var brush = new ImageBrush(bi)
        {
            Stretch = Stretch.Uniform,
            Opacity = 0.35 // lekko przeźroczyste, żeby widzieć rysunek
        };

        myCanvas.Background = brush;
    }
 
   
    private void UpdateChromaticityFromCurve(Func<double, double> intensity)
    {
        var (x, y) = spectrum.GetChromaticityFromIntensity(intensity);
        double w = myCanvas.ActualWidth, h = myCanvas.ActualHeight;

        Canvas.SetLeft(marker, x * w - marker.Width / 2);
        Canvas.SetTop(marker, (1 - y) * h - marker.Height / 2);

        if (!myCanvas.Children.Contains(marker))
            myCanvas.Children.Add(marker);

        var color = ChromaticityToSRGBColor(x, y, 1.0);
        DrawColorPatch(color);
    }
 
    
    private static Color ChromaticityToSRGBColor(double x, double y, double Y)
    {
        if (y <= 0) return Colors.Black;

        double X = (x / y) * Y;
        double Z = ((1 - x - y) / y) * Y;

        double rL =  3.2406 * X + (-1.5372) * Y + (-0.4986) * Z;
        double gL = -0.9689 * X +  1.8758  * Y +  0.0415  * Z;
        double bL =  0.0557 * X + (-0.2040) * Y +  1.0570  * Z;

        rL = Math.Clamp(rL, 0.0, 1.0);
        gL = Math.Clamp(gL, 0.0, 1.0);
        bL = Math.Clamp(bL, 0.0, 1.0);

        byte R = ToSRGB8(rL);
        byte G = ToSRGB8(gL);
        byte B = ToSRGB8(bL);

        return Color.FromRgb(R, G, B);
    }

    private static byte ToSRGB8(double u)
    {
        double v = (u <= 0.0031308) ? (12.92 * u) : (1.055 * Math.Pow(u, 1.0 / 2.4) - 0.055);
        v = Math.Clamp(v, 0.0, 1.0);
        return (byte)Math.Round(v * 255.0);
    }

    private void DrawColorPatch(Color c)
    {
        colorPatch.Fill = new SolidColorBrush(c);

        if (!myCanvas.Children.Contains(colorPatch))
            myCanvas.Children.Add(colorPatch);

        Canvas.SetLeft(colorPatch, 10);
        Canvas.SetTop (colorPatch, 10);
    }
    
    
    private void DrawBezierCurve(List<Point> points)
    {
        spectrumCanvas.Children.Clear();
        if (points.Count < 4) return;

        var pathFigure = new PathFigure { StartPoint = points[0] };
        var bezier = new BezierSegment(points[1], points[2], points[3], true);
        pathFigure.Segments.Add(bezier);

        var pathGeometry = new PathGeometry();
        pathGeometry.Figures.Add(pathFigure);

        var path = new Path
        {
            Stroke = Brushes.Blue,
            StrokeThickness = 2,
            Data = pathGeometry
        };

        spectrumCanvas.Children.Add(path);
    }


}
