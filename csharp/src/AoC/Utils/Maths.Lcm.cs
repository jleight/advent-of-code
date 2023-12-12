namespace AoC.Utils;

public static partial class Maths
{
    public static int Lcm(int a, int b)
        => a * b / Gcd(a, b);

    public static long Lcm(long a, long b)
        => a * b / Gcd(a, b);
}
