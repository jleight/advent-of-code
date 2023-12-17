namespace AoC.Y2023.Solutions;

public class D14P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var tiles = context
            .InputLines
            .Select(l => l.ToCharArray())
            .ToArray();

        tiles = Rotate(tiles);
        tiles = Tilt(tiles);

        var load = tiles
            .Select(l => l.Reverse().Select((t, i) => t == 'O' ? i + 1 : 0).Sum())
            .Sum();

        Console.WriteLine(load);
        return Task.CompletedTask;
    }

    private static char[][] Rotate(char[][] tiles)
    {
        var lines = new char[tiles[0].Length][];

        for (var i = 0; i < tiles[0].Length; i++)
        {
            lines[i] = new char[tiles.Length];

            for (var l = 0; l < tiles.Length; l++)
                lines[i][l] = tiles[l][^(i+1)];
        }

        return lines.ToArray();
    }

    private static char[][] Tilt(char[][] tiles)
    {
        var tilted = new char[tiles.Length][];

        for (var i = 0; i < tiles.Length; i++)
        {
            var chunks = tiles[i].ChunkBy(c => c == '#', true)
                .Select(c => $"{new string('O', c.Count(t => t == 'O'))}{new string('.', c.Count(t => t == '.'))}");

            tilted[i] = string
                .Join('#', chunks)
                .ToCharArray();
        }

        return tilted;
    }
}
