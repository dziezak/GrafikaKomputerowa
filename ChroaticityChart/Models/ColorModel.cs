namespace ChroaticityChart.Models;

public class ColorModel
{
    public double X { get; set; }
    public double Y { get; set; }
    public double Z { get; set; }
    
    public double CIE_x => (X+Y+Z) == 0 ? 0 : X / (X+Y+Z);
    public double CIE_y => (X+Y+Z) == 0 ? 0 : Y / (X+Y+Z);
    
}