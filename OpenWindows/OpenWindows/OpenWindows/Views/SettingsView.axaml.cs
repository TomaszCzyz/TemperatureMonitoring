using System.Globalization;
using Avalonia.Controls;
using OpenWindows.ViewModels;

namespace OpenWindows.Views;

public partial class SettingsView : UserControl
{
    public SettingsView()
    {
        DataContext = new SettingsViewModel();
        InitializeComponent();
    }

    private void Spinner_OnSpin(object? sender, SpinEventArgs e)
    {
        var btnSpinner = (ButtonSpinner)sender!;

        if (btnSpinner.Content is TextBlock textBlock && !string.IsNullOrEmpty(textBlock.Text))
        {
            var currentValue = float.Parse(textBlock.Text);

            if (e.Direction == SpinDirection.Increase)
            {
                currentValue += 1;
            }
            else
            {
                currentValue -= 1;
            }

            textBlock.Text = currentValue.ToString(CultureInfo.InvariantCulture);
        }
    }
}
