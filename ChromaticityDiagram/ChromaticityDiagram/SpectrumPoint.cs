namespace ChromaticityDiagram;

public class SpectrumPoint
{
    public double Lambda { get; set; } 
    public double X { get; set; }
    public double Y { get; set; }
    public double Z { get; set; }

    public (double x, double y) GetChromaticity()
    {
        double sum = X + Y + Z;
        return (X / sum, Y / sum);
    }
}
