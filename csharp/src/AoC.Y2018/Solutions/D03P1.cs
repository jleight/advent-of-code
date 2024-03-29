using System.Text.RegularExpressions;

namespace AoC.Y2018.Solutions;

public class D03P1 : ISolution
{
    private static readonly Regex _parser = new (@"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)");

    public Task Run(SolutionContext context)
    {
        var claims = context.InputLines
            .Select(Parse)
            .ToList();

        var maxX = claims.Max(c => c.X + c.DX);
        var maxY = claims.Max(c => c.Y + c.DY);

        var grid = new bool?[maxX, maxY];

        foreach (var claim in claims)
        {
            for (var x = claim.X; x < claim.X + claim.DX; x++)
            for (var y = claim.Y; y < claim.Y + claim.DY; y++)
                grid[x, y] = grid[x, y] != null;
        }

        var overlap = 0;
        for (var x = 0; x < maxX; x++)
        for (var y = 0; y < maxY; y++)
            overlap += grid[x, y] == true ? 1 : 0;

        Console.WriteLine(overlap);
        return Task.CompletedTask;
    }

    private static (int Id, int X, int Y, int DX, int DY) Parse(string claim)
    {
        var match = _parser.Match(claim);
        return (
            Convert.ToInt32(match.Groups[1].Value, 10),
            Convert.ToInt32(match.Groups[2].Value, 10),
            Convert.ToInt32(match.Groups[3].Value, 10),
            Convert.ToInt32(match.Groups[4].Value, 10),
            Convert.ToInt32(match.Groups[5].Value, 10)
        );
    }
}
