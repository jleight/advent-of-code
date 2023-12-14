using System.Collections;

namespace AoC.Collections;

public class Grid<T>(
        int width,
        int height,
        T defaultValue)
    : IEnumerable<(int X, int Y, T cell)>
{
    private readonly Dictionary<(int X, int Y), T> _cells = new();

    public int Width { get; } = width;
    public int Height { get; } = height;
    public int Count => _cells.Count;

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

    public IEnumerator<(int X, int Y, T cell)> GetEnumerator()
    {
        return _cells
            .Select(p => (p.Key.X, p.Key.Y, p.Value))
            .GetEnumerator();
    }

    IEnumerator IEnumerable.GetEnumerator()
        => GetEnumerator();
}

public static class Grid
{
    public static Grid<T> New<T>(int width, int height, T defaultValue)
        => new(width, height, defaultValue);
}

public static class GridExtensions
{
    public static void Print(this Grid<char> grid)
    {
        for (var y = 0; y < grid.Height; y++)
        {
            for (var x = 0; x < grid.Width; x++)
                Console.Write(grid[x, y]);
            Console.WriteLine();
        }
    }
}
