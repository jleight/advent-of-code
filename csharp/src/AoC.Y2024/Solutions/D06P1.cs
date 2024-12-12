namespace AoC.Y2024.Solutions;

public class D06P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = context
            .ToGrid('.');

        var (x, y, _) = grid
            .Single(c => c.Cell == '^');
        grid.Remove(x, y);

        var (dx, dy) = (0, -1);

        do
        {
            grid[x, y] = 'X';

            if (grid[x + dx, y + dy] == '#')
                (dx, dy) = (-dy, dx);

            (x, y) = (x + dx, y + dy);
        } while (grid.IsInBounds(x, y));

        var count = grid
            .Count(c => c.Cell == 'X');

        Console.WriteLine(count);
        return Task.CompletedTask;
    }
}
