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
        DrawAxes();
        Loaded += OnLoad;
        //SizeChanged += DrawAxes();
        
        myCanvas.SizeChanged += (_, __) => DrawChromaticityStuff();
        
    }

    private void OnLoad(object sender, RoutedEventArgs e)
    {
        spectrum = Spectrum.LoadFromFile("Data/wykres.txt");
        spectrumCurveView = new SpectrumCurveView(spectrumCanvas);
        spectrumCurveView.ChromaticityRequested += (_, intensityFunc) =>
        {
            //Console.WriteLine("Event fired!");
            UpdateChromaticityFromCurve(intensityFunc);
        };

        DrawChromaticityStuff();
    }
    
    

        private void DrawAxes()
        {
            double width = axesCanvas.ActualWidth;
            double height = axesCanvas.ActualHeight;

            // Jeśli jeszcze niezmierzone, użyj rozmiaru z kontenera
            if (double.IsNaN(width) || width == 0) width = axesCanvas.RenderSize.Width;
            if (double.IsNaN(height) || height == 0) height = axesCanvas.RenderSize.Height;

            // Fallback (np. przy pierwszym wywołaniu w bardzo wczesnym cyklu życia)
            if (width <= 0) width = 400;
            if (height <= 0) height = 400;

            axesCanvas.Children.Clear();

            // Marginesy osi
            const double left = 40;
            const double right = 20;
            const double top = 20;
            const double bottom = 40;

            // Oś X
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

            // Oś Y
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

            // (Opcjonalnie) cienkie linie siatki
            var gridBrush = new SolidColorBrush(Color.FromRgb(220, 220, 220));

            // Podziałki i etykiety co 10 (0..100)
            int divisions = 10;
            for (int i = 0; i <= divisions; i++)
            {
                double x = left + i * (width - left - right) / divisions;
                double y = height - bottom - i * (height - top - bottom) / divisions;

                // Podziałki na osi X
                axesCanvas.Children.Add(new Line
                {
                    X1 = x, Y1 = height - bottom,
                    X2 = x, Y2 = height - bottom + 5,
                    Stroke = Brushes.Black, StrokeThickness = 1
                });

                // Siatka pionowa (lekka)
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

                // Podziałki na osi Y
                axesCanvas.Children.Add(new Line
                {
                    X1 = left, Y1 = y,
                    X2 = left - 5, Y2 = y,
                    Stroke = Brushes.Black, StrokeThickness = 1
                });

                // Siatka pozioma (lekka)
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

            // Opisy osi
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

        // Dodaj punkt (czerwona kropka)
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
            Opacity = 0.35 
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
    
    



}
