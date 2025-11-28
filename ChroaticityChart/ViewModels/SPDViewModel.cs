using CommunityToolkit.Mvvm.ComponentModel;
using ChroaticityChart.Models;
using System.Collections.ObjectModel;

namespace ChroaticityChart.ViewModels
{
    public partial class SPDViewModel : ObservableObject
    {
        public ObservableCollection<double> Wavelengths { get; set; } = new ObservableCollection<double>();
        public ObservableCollection<double> Values { get; set; } = new ObservableCollection<double>();

        [ObservableProperty]
        private ColorModel currentColor = new ColorModel();

        public SPDViewModel()
        {
            // przykładowe dane
            for (double wl = 380; wl <= 780; wl += 5)
            {
                Wavelengths.Add(wl);
                Values.Add(0);
            }
        }

        // Funkcja do aktualizacji kolorów w CIE xy na podstawie SPD
        public void UpdateColor(ColorModel color)
        {
            CurrentColor = color;
        }
    }
}
