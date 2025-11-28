using System.Collections.ObjectModel;
using ChroaticityChart.ViewModels;
using CommunityToolkit.Mvvm.ComponentModel;

namespace ChroaticityChart.ViewModels
{
    public partial class MainWindowViewModel : ObservableObject
    {
            [ObservableProperty]
            private string _title = "Avalonia MVVM + SPD/CIE";

            public SPDViewModel SPDViewModel { get; set; }

            public MainWindowViewModel()
            {
                SPDViewModel = new SPDViewModel();
            }
     
    }
}