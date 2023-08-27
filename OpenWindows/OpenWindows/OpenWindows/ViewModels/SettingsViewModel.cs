using System;
using CommunityToolkit.Mvvm.ComponentModel;

namespace OpenWindows.ViewModels;

public partial class SettingsViewModel : ViewModelBase
{
    [ObservableProperty]
    private TimeSpan _weatherCheckFrequency = TimeSpan.FromMinutes(5);

    [ObservableProperty]
    private float _latitude = 50.5f;

    [ObservableProperty]
    private float _longitude = 60.5f;
}
