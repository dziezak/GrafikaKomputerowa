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
    
    public MainWindow()
    {
        InitializeComponent();
        DrawAxes();
        Loaded += OnLoad;
        SetBackgroundFromResource("podkowa.jpg");
        myCanvas.SizeChanged += (_, __) => DrawChromaticityStuff();
        
    }

    private void OnLoad(object sender, RoutedEventArgs e)
    {
        spectrum = Spectrum.LoadFromFile("Data/wykres.txt");
        spectrumCurveView = new SpectrumCurveView(spectrumCanvas);
        spectrumCurveView.ChromaticityRequested += (_, intensityFunc) =>
        {
            UpdateChromaticityFromCurve(intensityFunc);
        };

        DrawChromaticityStuff();
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


    private void DrawChromaticityStuff()
    {
        if (myCanvas.ActualWidth <= 0 || myCanvas.ActualHeight <= 0 || spectrum == null) return;

        myCanvas.Children.Clear();
        DrawSpectrumCanvas(myCanvas, spectrum);  // brzeg podkowy (kolorowy)
        DrawSRGBGamut(myCanvas);                 // trójkąt gamutu sRGB

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
        double W = canvas.ActualWidth;
        double H = canvas.ActualHeight;

        if (W <= 0 || H <= 0 || spectrum == null || spectrum.Points.Count == 0)
            return;

        canvas.Children.Clear();

        // ---- TU JEST KLUCZ ----
        double cieOriginX = 0.20 * W;  // miejsce gdzie jest x=0
        double cieOriginY = 0.82 * H;  // miejsce gdzie jest y=0
        double cieMaxX    = 1 * W;  // miejsce gdzie jest x=1
        double cieMaxY    = 0.01 * H;  // miejsce gdzie jest y=1

        Point? prev = null;
        double prevLambda = 0;

        foreach (var s in spectrum.Points)
        {
            double sum = s.X + s.Y + s.Z;
            if (sum <= 0) continue;

            double x = s.X / sum;
            double y = s.Y / sum;

            // Przeliczenie na piksele z uwzględnieniem rzeczywistego położenia osi
            double px = cieOriginX + x * (cieMaxX - cieOriginX);
            double py = cieOriginY - y * (cieOriginY - cieMaxY);

            Point p = new Point(px, py);

            if (prev.HasValue)
            {
                var line = new Line
                {
                    X1 = prev.Value.X,
                    Y1 = prev.Value.Y,
                    X2 = p.X,
                    Y2 = p.Y,
                    Stroke = new SolidColorBrush(WavelengthToRGB(prevLambda)),
                    StrokeThickness = 2
                };
                canvas.Children.Add(line);
            }

            prev = p;
            prevLambda = s.Lambda;
        }
    }



    
    private void ComputeCieTransform(Canvas canvas, out double originX, out double originY, out double maxX, out double maxY)
    {
        double W = canvas.ActualWidth;
        double H = canvas.ActualHeight;

        originX = 0.20 * W;
        originY = 0.82 * H;
        maxX    = 1.00 * W;
        maxY    = 0.01 * H;

    }

    // Mapuje (x,y) chromatyczności (w zakresie 0..1) na współrzędne piksela canvasu
    private Point MapChromaticityToPixel(double x, double y, Canvas canvas)
    {
        ComputeCieTransform(canvas, out double originX, out double originY, out double maxX, out double maxY);

        double px = originX + x * (maxX - originX);
        double py = originY - y * (originY - maxY); // minus bo y rośnie w górę w układzie CIE, ale w dół w pikselach
        return new Point(px, py);
    }

    private void DrawSRGBGamut(Canvas canvas)
    {
        // usuń ewentualne stare elementy związane tylko z gamutem,
        // lub po prostu dopisz – tutaj dodajemy nowe elementy
        double w = canvas.ActualWidth;
        double h = canvas.ActualHeight;
        if (w <= 0 || h <= 0) return;

        // punkty gamutu sRGB w współrzędnych chromatyczności
        var rgb = new[] {
            (x:0.640, y:0.330),
            (x:0.300, y:0.600),
            (x:0.150, y:0.060)
        };

        // utwórz polygon i wypełnienie
        var polygon = new Polygon
        {
            Stroke = Brushes.Gray,
            StrokeThickness = 1.5,
            Fill = new SolidColorBrush(Color.FromArgb(30, 128, 128, 128)),
            IsHitTestVisible = false
        };

        foreach (var p in rgb)
        {
            var pt = MapChromaticityToPixel(p.x, p.y, canvas);
            polygon.Points.Add(pt);
        }

        canvas.Children.Add(polygon);

        // Whitepoint D65
        var wp = (x:0.3127, y:0.3290);
        var wpMarker = new Ellipse { Width=6, Height=6, Fill=Brushes.White, Stroke=Brushes.Black, StrokeThickness=1, IsHitTestVisible = false };
        var wpPixel = MapChromaticityToPixel(wp.x, wp.y, canvas);
        Canvas.SetLeft(wpMarker, wpPixel.X - wpMarker.Width / 2);
        Canvas.SetTop (wpMarker, wpPixel.Y - wpMarker.Height / 2);
        canvas.Children.Add(wpMarker);
    }


    private void SetBackgroundFromResource(string resourcePath)
    {
        try
        {
            var uri = new Uri(resourcePath, UriKind.Relative);
            var bi = new BitmapImage(uri);

            var brush = new ImageBrush(bi)
            {
                Stretch = Stretch.Fill, // zachowuje proporcje
                AlignmentX = AlignmentX.Center,
                AlignmentY = AlignmentY.Center,
                Opacity = 1 // przezroczystość
            };

            myCanvas.Background = brush;
        }
        catch (Exception ex)
        {
            MessageBox.Show($"Nie udało się ustawić tła: {ex.Message}");
        }
    }


 
   
    private void UpdateChromaticityFromCurve(Func<double, double> intensity)
    {
        var (x, y) = spectrum.GetChromaticityFromIntensity(intensity);
        double w = myCanvas.ActualWidth, h = myCanvas.ActualHeight;

        Point p = MapChromaticityToPixel(x, y, myCanvas);

        Canvas.SetLeft(marker, p.X - marker.Width / 2);
        Canvas.SetTop(marker, p.Y - marker.Height / 2);


        if (!myCanvas.Children.Contains(marker))
            myCanvas.Children.Add(marker);

        var color = ChromaticityToSRGBColor(x, y, 1.0);
        DrawColorPatch(color);
    }
 
    
    public Color ChromaticityToSRGBColor(double x, double y, double Y = 1.0)
    {
        // 1. Rekonstrukcja XYZ z (x,y,Y)
        if (y <= 0) return Color.FromRgb(0, 0, 0); 
        double X = (x / y) * Y;
        double Z = ((1 - x - y) / y) * Y;

        // 2. XYZ → Linear sRGB
        double r_lin =  3.2406 * X - 1.5372 * Y - 0.4986 * Z;
        double g_lin = -0.9689 * X + 1.8758 * Y + 0.0415 * Z;
        double b_lin =  0.0557 * X - 0.2040 * Y + 1.0570 * Z;

        // 3. Korekta gamma sRGB (OETF) i konwersja na 8-bit
    
        // Korekcja gamma jest stosowana do wartości liniowych
        double r_srgb = Gamma(r_lin);
        double g_srgb = Gamma(g_lin);
        double b_srgb = Gamma(b_lin);

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

    private double Gamma(double c)
    {
        if (c <= 0.0031308) return 12.92 * c;
        return 1.055 * Math.Pow(c, 1.0 / 2.4) - 0.055;
    }


    private static byte ToSRGB8(double u)
    {
        // Jeśli u jest już po korekcji gamma, to tylko przypinamy i konwertujemy
        // Twoja obecna funkcja ToSRGB8 wykonuje RÓWNIEŻ korekcję gamma, co jest BŁĘDEM
    
        // Zastąp starą implementację ToSRGB8 tą uproszczoną:
        double v = Math.Clamp(u, 0.0, 1.0);
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
    
    



}
