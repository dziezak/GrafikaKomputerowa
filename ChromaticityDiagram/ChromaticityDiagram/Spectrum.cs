using System.Collections;
using System.IO;

namespace ChromaticityDiagram;



public class Spectrum
{
    public List<SpectrumPoint> Points { get; set; } = new();

    public static Spectrum LoadFromFile(string path)
    {
        var spectrum = new Spectrum();

        var lines = File.ReadAllLines(path).Skip(1);

        int lineNumber = 0;
        foreach (var line in lines)
        {
            lineNumber++;
            if (string.IsNullOrWhiteSpace(line)) continue;
            var parts = line.Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length >= 4)
            {
                spectrum.Points.Add(new SpectrumPoint
                {
                    Lambda = double.Parse(parts[0], System.Globalization.CultureInfo.InvariantCulture),
                    X = double.Parse(parts[1], System.Globalization.CultureInfo.InvariantCulture),
                    Y = double.Parse(parts[2], System.Globalization.CultureInfo.InvariantCulture),
                    Z = double.Parse(parts[3], System.Globalization.CultureInfo.InvariantCulture)
                });
            }
        }

        return spectrum;
    }


    public (double x, double y) GetChromaticityFromIntensity(Func<double,double> intensity)
    {
        double Xsum = 0, Ysum = 0, Zsum = 0;
        foreach (var p in Points)
        {
            double I = intensity(p.Lambda);
            Xsum += I * p.X;
            Ysum += I * p.Y;
            Zsum += I * p.Z;
        }
        double sum = Xsum + Ysum + Zsum;
        return (Xsum / sum, Ysum / sum);
    }
}
