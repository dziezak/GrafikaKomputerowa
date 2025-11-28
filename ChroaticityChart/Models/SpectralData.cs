using System.Collections.Generic;

namespace ChroaticityChart.Models
{
namespace MyApp.Models
{
    public class SpectralData
    {
        public List<double> Wavelengths { get; set; } = new List<double>();
        public List<double> Values { get; set; } = new List<double>();

        public SpectralData() { }

        public SpectralData(List<double> wavelengths, List<double> values)
        {
            Wavelengths = wavelengths;
            Values = values;
        }

        // Możemy dodać funkcję interpolacji, normalizacji itd.
    }
}
    
}
