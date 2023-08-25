using System;
using System.Collections.Generic;

namespace OpenWindows.ViewModels;

public sealed class SettingsEntry
{
    public required string Name { get; init; }
    public required object Value { get; set; }
}

public class SettingsViewModel : ViewModelBase
{
    public List<SettingsEntry> Settings { get; set; }
        = new()
        {
            new SettingsEntry { Name = "Weather check frequency", Value = TimeSpan.FromMinutes(5) },
            new SettingsEntry { Name = "Latitude", Value = 50.5f },
            new SettingsEntry { Name = "Longitude", Value = 60.5f },
        };
}