namespace OpenWindows.ViewModels;

public class MainViewModel : ViewModelBase
{
    public WelcomeViewModel WelcomeViewModel { get; } = new();

    public SettingsViewModel SettingsViewModel { get; } = new();
}