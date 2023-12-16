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

    public static IEnumerable<T> Repeat<T>(
        this ICollection<T> source)
    {
        while (true)
        {
            foreach (var item in source)
                yield return item;
        }
    }

    public static IEnumerable<T[]> ChunkBy<T>(
        this IEnumerable<T> source,
        Func<T, bool> predicate)
    {
        var chunk = new List<T>();

        foreach (var item in source)
        {
            if (!predicate(item))
            {
                chunk.Add(item);
                continue;
            }

            if (chunk.Count <= 0)
                continue;

            yield return chunk.ToArray();
            chunk.Clear();
        }

        if (chunk.Count > 0)
            yield return chunk.ToArray();
    }
}
