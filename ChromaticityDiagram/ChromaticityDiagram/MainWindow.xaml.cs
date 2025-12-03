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

public partial class MainWindow : Window
{
    private Spectrum spectrum;
    private SpectrumCurveView spectrumCurveView;
    private Ellipse marker = new Ellipse { Width = 8, Height = 8, Fill = Brushes.Black, Stroke = Brushes.White, StrokeThickness = 1 };
    private Rectangle colorPatch = new Rectangle {Width = 80, Height = 40, Stroke = Brushes.Black, StrokeThickness = 1};
    public List<Point> cieBoundaryCanvas = new List<Point>();
    private CieCanvasTransform cieTransform;
    public enum GamutMapping { Clip, Normalize }
 
    public MainWindow()
    {
        InitializeComponent();
        DrawAxes();
        Loaded += OnLoad;
        //myCanvas.SizeChanged += (_, __) => DrawChromaticityStuff();
        myCanvas.SizeChanged += (_, __) => DrawSpectrumCanvas(myCanvas, spectrum);
        myCanvas.SizeChanged += (_, __) => DrawChromaticityFill(myCanvas, GamutMapping.Clip);
    }

    private void OnLoad(object sender, RoutedEventArgs e)
    {
        spectrum = Spectrum.LoadFromFile("Data/wykres.txt");
        spectrumCurveView = new SpectrumCurveView(spectrumCanvas);
        spectrumCurveView.ChromaticityRequested += (_, intensityFunc) =>
        {
            UpdateChromaticityFromCurve(intensityFunc); // point on ChromaticityDiagram
        };

        DrawSpectrumCanvas(myCanvas, spectrum);
        DrawChromaticityFill(myCanvas, GamutMapping.Clip);
    }

    private void DrawAxes()
    {
        double width = axesCanvas.ActualWidth;
        double height = axesCanvas.ActualHeight;

        if (double.IsNaN(width) || width == 0) width = axesCanvas.RenderSize.Width;
        if (double.IsNaN(height) || height == 0) height = axesCanvas.RenderSize.Height;

        if (width <= 0) width = 400;
        if (height <= 0) height = 400;

        axesCanvas.Children.Clear();

        const double left = 40;
        const double right = 20;
        const double top = 20;
        const double bottom = 40;

        Line xAxis = new Line
        {
            X1 = left,
            Y1 = height - bottom,
            X2 = width - right,
            Y2 = height - bottom,
            Stroke = Brushes.Black,
            StrokeThickness = 2
        };
        axesCanvas.Children.Add(xAxis);

        Line yAxis = new Line
        {
            X1 = left,
            Y1 = height - bottom,
            X2 = left,
            Y2 = top,
            Stroke = Brushes.Black,
            StrokeThickness = 2
        };
        axesCanvas.Children.Add(yAxis);

        var gridBrush = new SolidColorBrush(Color.FromRgb(220, 220, 220));

        int divisions = 10;
        for (int i = 0; i <= divisions; i++)
        {
            double x = left + i * (width - left - right) / divisions;
            double y = height - bottom - i * (height - top - bottom) / divisions;

            axesCanvas.Children.Add(new Line
            {
                X1 = x, Y1 = height - bottom,
                X2 = x, Y2 = height - bottom + 5,
                Stroke = Brushes.Black, StrokeThickness = 1
            });

            if (i > 0 && i < divisions)
            {
                axesCanvas.Children.Add(new Line
                {
                    X1 = x, Y1 = height - bottom,
                    X2 = x, Y2 = top,
                    Stroke = gridBrush, StrokeThickness = 0.5
                });
            }

            var labelX = new TextBlock
            {
                Text = (i * 10).ToString(),
                FontSize = 12
            };
            Canvas.SetLeft(labelX, x - 10);
            Canvas.SetTop(labelX, height - bottom + 8);
            axesCanvas.Children.Add(labelX);

            axesCanvas.Children.Add(new Line
            {
                X1 = left, Y1 = y,
                X2 = left - 5, Y2 = y,
                Stroke = Brushes.Black, StrokeThickness = 1
            });

            if (i > 0 && i < divisions)
            {
                axesCanvas.Children.Add(new Line
                {
                    X1 = left, Y1 = y,
                    X2 = width - right, Y2 = y,
                    Stroke = gridBrush, StrokeThickness = 0.5
                });
            }

            var labelY = new TextBlock
            {
                Text = (i * 10).ToString(),
                FontSize = 12
            };
            Canvas.SetLeft(labelY, left - 32);
            Canvas.SetTop(labelY, y - 8);
            axesCanvas.Children.Add(labelY);
        }

        var xTitle = new TextBlock { Text = "X", FontWeight = FontWeights.Bold };
        Canvas.SetLeft(xTitle, width - right - 10);
        Canvas.SetTop(xTitle, height - bottom + 24);
        axesCanvas.Children.Add(xTitle);

        var yTitle = new TextBlock { Text = "Y", FontWeight = FontWeights.Bold };
        Canvas.SetLeft(yTitle, left - 18);
        Canvas.SetTop(yTitle, top - 4);
        axesCanvas.Children.Add(yTitle);
    }

    
    private void SpectrumCanvas_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        Point clickPoint = e.GetPosition(spectrumCanvas);

        Ellipse dot = new Ellipse
        {
            Width = 8,
            Height = 8,
            Fill = Brushes.Red
        };
        Canvas.SetLeft(dot, clickPoint.X - 4);
        Canvas.SetTop(dot, clickPoint.Y - 4);
        spectrumCanvas.Children.Add(dot);
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
        double W = canvas.ActualWidth;
        double H = canvas.ActualHeight;

        if (W <= 0 || H <= 0 || spectrum == null || spectrum.Points.Count == 0)
            return;

        canvas.Children.Clear();
        cieBoundaryCanvas.Clear();

        // oblicz transformację poprawnie (statycznie)
        cieTransform = CieCanvasTransform.ComputeCieCanvasTransform(canvas);

        Point? prev = null;

        // upewniamy się, że punkty spektralne są w kolejności lambda rosnąco
        var ordered = spectrum.Points.OrderBy(p => p.Lambda).ToList();

        foreach (var s in ordered)
        {
            double sum = s.X + s.Y + s.Z;
            if (sum <= 0) continue;

            double x = s.X / sum;
            double y = s.Y / sum;

            Point p = cieTransform.Map(x, y);

            cieBoundaryCanvas.Add(p);

            if (prev.HasValue)
            {
                var line = new Line
                {
                    X1 = prev.Value.X,
                    Y1 = prev.Value.Y,
                    X2 = p.X,
                    Y2 = p.Y,
                    Stroke = new SolidColorBrush(WavelengthToRGB(s.Lambda)),
                    StrokeThickness = 2,
                    IsHitTestVisible = false
                };
                canvas.Children.Add(line);
            }

            prev = p;
        }

        // zamknięcie obwiedni - dodaj pierwszy punkt na końcu, ale nie duplikuj rysowania linii
        if (cieBoundaryCanvas.Count > 2)
            cieBoundaryCanvas.Add(cieBoundaryCanvas[0]);
    }


    
public void DrawChromaticityFill(Canvas canvas, GamutMapping mapping = GamutMapping.Normalize)
{
    if (canvas == null || spectrum == null || spectrum.Points.Count == 0)
        return;

    // upewnij się, że transform jest obliczona
    if (cieTransform.Scale == 0)
        cieTransform = CieCanvasTransform.ComputeCieCanvasTransform(canvas);


    int w = (int)Math.Max(1, canvas.ActualWidth);
    int h = (int)Math.Max(1, canvas.ActualHeight);

    var wb = new WriteableBitmap(w, h, 96, 96, PixelFormats.Bgra32, null);
    int stride = w * 4;
    byte[] pixels = new byte[h * stride];

    Parallel.For(0, h, py =>
    {
        int rowStart = py * stride;
        for (int px = 0; px < w; px++)
        {
            // test czy piksel leży wewnątrz podkowy (PointInPolygon na cieBoundaryCanvas w pikselach)
            if (!PointInPolygon(cieBoundaryCanvas, px, py))
            {
                int idx = rowStart + px * 4;
                pixels[idx + 0] = 0;
                pixels[idx + 1] = 0;
                pixels[idx + 2] = 0;
                pixels[idx + 3] = 0;
                continue;
            }

            // zamiana px,py -> x,y wykorzystując InverseMap
            var (x, y) = cieTransform.InverseMap(px, py);

            // oblicz linear RGB z xyY
            var (r_lin, g_lin, b_lin) = ChromaticityToLinearSRGB(x, y, 1.0);

            // mapowanie gamutu jak wcześniej
            if (mapping == GamutMapping.Clip)
            {
                r_lin = Math.Clamp(r_lin, 0.0, 1.0);
                g_lin = Math.Clamp(g_lin, 0.0, 1.0);
                b_lin = Math.Clamp(b_lin, 0.0, 1.0);
            }
            else
            {
                r_lin = Math.Max(0.0, r_lin);
                g_lin = Math.Max(0.0, g_lin);
                b_lin = Math.Max(0.0, b_lin);
                double maxv = Math.Max(r_lin, Math.Max(g_lin, b_lin));
                if (maxv > 0) { r_lin /= maxv; g_lin /= maxv; b_lin /= maxv; }
                else { int idx = rowStart + px * 4; pixels[idx + 3] = 0; continue; }
            }

            double r_srgb = GammaSRGB(r_lin);
            double g_srgb = GammaSRGB(g_lin);
            double b_srgb = GammaSRGB(b_lin);

            int index = rowStart + px * 4;
            pixels[index + 0] = ClampAndByte(b_srgb);
            pixels[index + 1] = ClampAndByte(g_srgb);
            pixels[index + 2] = ClampAndByte(r_srgb);
            pixels[index + 3] = 255;
        }
    });

    Int32Rect rect = new Int32Rect(0, 0, w, h);
    wb.WritePixels(rect, pixels, stride, 0);

    Application.Current.Dispatcher.Invoke(() =>
    {
        var img = new System.Windows.Controls.Image
        {
            Source = wb,
            Width = w,
            Height = h,
            Stretch = Stretch.None,
            IsHitTestVisible = false,
            Tag = "ChromaticityFill"
        };

        // usuń stare i wstaw nowe na spód
        for (int i = canvas.Children.Count - 1; i >= 0; i--)
        {
            if (canvas.Children[i] is System.Windows.Controls.Image oldImg && oldImg.Tag as string == "ChromaticityFill")
                canvas.Children.RemoveAt(i);
        }
        Canvas.SetLeft(img, 0);
        Canvas.SetTop(img, 0);
        canvas.Children.Insert(0, img);
    });
}


    private Point MapChromaticityToPixel(double x, double y)
    {
        return cieTransform.Map(x, y);
    }
   
    private void UpdateChromaticityFromCurve(Func<double, double> intensity)
    {
        var (x, y) = spectrum.GetChromaticityFromIntensity(intensity);

        Point p = MapChromaticityToPixel(x, y);

        Canvas.SetLeft(marker, p.X - marker.Width / 2);
        Canvas.SetTop(marker, p.Y - marker.Height / 2);

        if (!myCanvas.Children.Contains(marker))
            myCanvas.Children.Add(marker);

        var color = ChromaticityToSRGBColor(x, y, 1.0);
        DrawColorPatch(color);
    }
 
    
    public Color ChromaticityToSRGBColor(double x, double y, double Y = 1.0)
    {
        if (y <= 0) return Color.FromRgb(0, 0, 0); 
        double X = (x / y) * Y;
        double Z = ((1 - x - y) / y) * Y;

        double r_lin =  3.2406 * X - 1.5372 * Y - 0.4986 * Z;
        double g_lin = -0.9689 * X + 1.8758 * Y + 0.0415 * Z;
        double b_lin =  0.0557 * X - 0.2040 * Y + 1.0570 * Z;
    
        r_lin = Math.Max(0, r_lin);
        g_lin = Math.Max(0, g_lin);
        b_lin = Math.Max(0, b_lin);
    
        double max = Math.Max(r_lin, Math.Max(g_lin, b_lin));
        if (max > 0)
        {
            r_lin /= max;
            g_lin /= max;
            b_lin /= max;
        }

        double r_srgb = GammaSRGB(r_lin);
        double g_srgb = GammaSRGB(g_lin);
        double b_srgb = GammaSRGB(b_lin);

        return Color.FromRgb(
            ClampAndConvertToByte(r_srgb),
            ClampAndConvertToByte(g_srgb),
            ClampAndConvertToByte(b_srgb)
        );
    }

    
    private static byte ClampAndConvertToByte(double value)
    {
        double clamped = Math.Clamp(value, 0.0, 1.0);
        return (byte)Math.Round(clamped * 255.0);
    }

    private void DrawColorPatch(Color c)
    {
        colorPatch.Fill = new SolidColorBrush(c);

        if (!myCanvas.Children.Contains(colorPatch))
            myCanvas.Children.Add(colorPatch);

        Canvas.SetLeft(colorPatch, 10);
        Canvas.SetTop (colorPatch, 10);
    }
    
    
    
    private double GammaSRGB(double c)
    {
        if (c <= 0.0031308) return 12.92 * c;
        return 1.055 * Math.Pow(c, 1.0 / 2.4) - 0.055;
    }

    private static byte ClampAndByte(double v)
    {
        double c = Math.Clamp(v, 0.0, 1.0);
        return (byte)Math.Round(c * 255.0);
    }

    private (double r, double g, double b) ChromaticityToLinearSRGB(double x, double y, double Y = 1.0)
    {
        if (y <= 0) return (0, 0, 0);
        double X = (x / y) * Y;
        double Z = ((1 - x - y) / y) * Y;

        double r_lin =  3.2406 * X - 1.5372 * Y - 0.4986 * Z;
        double g_lin = -0.9689 * X + 1.8758 * Y + 0.0415 * Z;
        double b_lin =  0.0557 * X - 0.2040 * Y + 1.0570 * Z;

        return (r_lin, g_lin, b_lin);
    }



    
    public static bool PointInPolygon(List<Point> poly, double x, double y)
    {
        bool inside = false;
        int count = poly.Count;

        for (int i = 0, j = count - 1; i < count; j = i++)
        {
            var pi = poly[i];
            var pj = poly[j];

            bool intersect = 
                ((pi.Y > y) != (pj.Y > y)) &&
                (x < (pj.X - pi.X) * (y - pi.Y) / (pj.Y - pi.Y) + pi.X);

            if (intersect)
                inside = !inside;
        }
        return inside;
    }

    
}
