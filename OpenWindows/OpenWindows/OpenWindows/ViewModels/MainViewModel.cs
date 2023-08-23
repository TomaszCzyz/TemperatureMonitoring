using System.Collections.Generic;

namespace OpenWindows.ViewModels;

public enum SettingType
{
    Text,
    Number,
    SingleChoice,
    MultipleChoice,
}

public class MainViewModel : ViewModelBase
{
    public IEnumerable<(string SettingName, SettingType SettingType)> Settings { get; set; }
        = new[]
        {
            ("temp freq", SettingType.Text),
            ("Latitude", SettingType.MultipleChoice),
            ("Longitude", SettingType.SingleChoice)
        };
}