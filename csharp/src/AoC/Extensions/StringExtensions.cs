namespace AoC.Extensions;

public static class StringExtensions
{
    public static T ConvertTo<T>(this string value)
        => (T)Convert.ChangeType(value, typeof(T));
}
