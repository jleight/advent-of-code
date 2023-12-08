namespace AoC.Collections;

public class Grid<T>(
    int width,
    int height,
    T defaultValue)
{
    private readonly Dictionary<(int X, int Y), T> _cells = new();

    public int Width { get; } = width;
    public int Height { get; } = height;

    public T this[int x, int y]
    {
        get => _cells.GetValueOrDefault((x, y), defaultValue);
        set => _cells[(x, y)] = value;
    }

    public IEnumerable<TOut> Select<TOut>(
        Func<(int X, int Y), T, TOut> action,
        bool skipEmpty = true)
    {
        if (skipEmpty)
        {
            foreach (var (key, value) in _cells)
                yield return action(key, value);
        }
        else
        {
            for (var y = 0; y < Height; y++)
            for (var x = 0; x < Width; x++)
                yield return action((x, y), this[x, y]);
        }
    }
}

public static class Grid
{
    public static Grid<T> New<T>(int width, int height, T defaultValue)
        => new(width, height, defaultValue);
}
