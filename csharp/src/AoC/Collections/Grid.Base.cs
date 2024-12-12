using System.Collections;

namespace AoC.Collections;

public class Grid<T>(
        int width,
        int height,
        T defaultValue)
    : IEnumerable<(int X, int Y, T Cell)>
{
    private readonly Dictionary<(int X, int Y), T> _cells = new();
    private readonly T _default = defaultValue;

    public int Width { get; } = width;
    public int Height { get; } = height;
    public int Count => _cells.Count;

    private Grid(Grid<T> other)
        : this(other.Width, other.Height, other._default)
    {
        _cells = new Dictionary<(int X, int Y), T>(other._cells);
    }

    public bool IsDefault(int x, int y)
        => !_cells.ContainsKey((x, y));

    public bool IsInBounds(int x, int y)
        => x >= 0 && x < Width && y >= 0 && y < Height;

    public bool Remove(int x, int y)
        => _cells.Remove((x, y));

    public Grid<T> Clone()
        => new(this);

    public T this[int x, int y]
    {
        get => _cells.GetValueOrDefault((x, y), _default);
        set
        {
            if (Comparer<T>.Default.Compare(value, _default) == 0)
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
