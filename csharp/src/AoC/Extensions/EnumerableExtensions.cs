namespace AoC.Extensions;

public static class EnumerableExtensions
{
    public static IEnumerable<T> Tap<T>(
        this IEnumerable<T> source,
        Action<T> tap)
    {
        return source
            .Select(x =>
            {
                tap(x);
                return x;
            });
    }

    public static IEnumerable<T> Tap<T>(
        this IEnumerable<T> source,
        Action<string> tap)
    {
        return source
            .Select(x =>
            {
                if (x is null)
                    tap("<null>");
                else if (x is string s)
                    tap(s);
                else
                    tap(Convert.ToString(x) ?? "<not convertable to string>");
                return x;
            });
    }
}
