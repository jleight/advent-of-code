namespace AoC.Y2024.Solutions;

public class D06P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = context
            .ToGrid('.');

        var (ix, iy, _) = grid
            .Single(c => c.Cell == '^');
        grid.Remove(ix, iy);

        var count = 0;

        for (var oy = 0; oy < grid.Height; oy++)
        for (var ox = 0; ox < grid.Width; ox++)
        {
            if (ox == ix && oy == iy)
                continue;
            if (!grid.IsDefault(ox, oy))
                continue;

            var test = grid.Clone();
            test[ox, oy] = '#';

            var (x, y, dx, dy) = (ix, iy, 0, -1);
            var moves = new HashSet<(int X, int Y, int Dx, int Dy)>();

            do
            {
                if (!moves.Add((x, y, dx, dy)))
                {
                    count += 1;
                    break;
                }

                if (test[x + dx, y + dy] == '#')
                    (dx, dy) = (-dy, dx);
                else
                    (x, y) = (x + dx, y + dy);
            } while (test.IsInBounds(x, y));
        }

        Console.WriteLine(count);
        return Task.CompletedTask;
    }
}
