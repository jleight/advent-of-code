namespace AoC.Utils;

public static partial class Maths
{
    public static string ToBase(
        int value,
        params ReadOnlySpan<char> digits)
    {
        var i = 32;
        var buffer = new char[i];

        do
        {
            buffer[--i] = digits[value % digits.Length];
            value /= digits.Length;
        }
        while (value > 0);

        var result = new char[32 - i];
        Array.Copy(buffer, i, result, 0, 32 - i);

        return new string(result);
    }
}
