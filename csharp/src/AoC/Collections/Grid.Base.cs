using System.Collections;

namespace AoC.Collections;

public class Grid<T>(
        int width,
        int height,
        T defaultValue)
    : IEnumerable<(int X, int Y, T Cell)>
{
    private readonly Dictionary<(int X, int Y), T> _cells = new();

    public int Width { get; } = width;
    public int Height { get; } = height;
    public int Count => _cells.Count;

    public T this[int x, int y]
    {
        get => _cells.GetValueOrDefault((x, y), defaultValue);
        set
        {
            if (Comparer<T>.Default.Compare(value, defaultValue) == 0)
                _cells.Remove((x, y));
            else
                _cells[(x, y)] = value;
        }
    }

    public IEnumerator<(int X, int Y, T Cell)> GetEnumerator()
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
