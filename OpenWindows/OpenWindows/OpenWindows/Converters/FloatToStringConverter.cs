using System;
using System.Globalization;
using Avalonia.Data.Converters;

namespace OpenWindows.Converters;

public class FloatToStringConverter : IValueConverter
{
    public object? Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        return value is float f ? f.ToString(CultureInfo.InvariantCulture) : null;
    }

    public object? ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        return value is string s ? float.Parse(s) : null;
    }
}