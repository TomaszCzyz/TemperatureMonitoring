using Avalonia.Controls;

namespace OpenWindows.Views;

public partial class SettingsView : UserControl
{
    public SettingsView()
    {
        InitializeComponent();
    }

    private void Spinner_OnSpin(object? sender, SpinEventArgs e)
    {
        var buttonSpinner = sender;
    }
}