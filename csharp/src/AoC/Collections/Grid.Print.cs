namespace AoC.Collections;

public static partial class GridExtensions
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
