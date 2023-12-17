namespace AoC.Y2023.Solutions;

public class D14P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var tiles = context
            .InputLines
            .Select(l => l.ToCharArray())
            .ToArray();

        var cache = new Dictionary<string, long>();

        var cycleStart = 0L;
        var cycleLength = 0L;

        for (var i = 0L; i < 1000000000L; i++)
        {
            var key = string.Join("", tiles.SelectMany(t => t));

            if (cache.TryGetValue(key, out var originalIndex))
            {
                cycleStart = originalIndex;
                cycleLength = i - originalIndex;
                break;
            }

            cache[key] = i;
            tiles = Cycle(tiles);
        }

        if (cycleLength > 0)
        {
            var remaining = (1000000000L - cycleStart) % cycleLength;
            for (var i = 0L; i < remaining; i++)
                tiles = Cycle(tiles);
        }

        tiles = Rotate(tiles);

        var load = tiles
            .Select(l => l.Reverse().Select((t, i) => t == 'O' ? i + 1 : 0).Sum())
            .Sum();

        Console.WriteLine(load);
        return Task.CompletedTask;
    }

    private static char[][] Cycle(char[][] tiles)
    {
        tiles = Rotate(tiles); // > N
        tiles = Tilt(tiles);   // tilt N

        tiles = Rotate(tiles); // v N
        tiles = Rotate(tiles); // > N
        tiles = Rotate(tiles); // ^ N
        tiles = Tilt(tiles);   // tilt W

        tiles = Rotate(tiles); // < N
        tiles = Rotate(tiles); // v N
        tiles = Rotate(tiles); // > N
        tiles = Tilt(tiles);   // tilt S

        tiles = Rotate(tiles); // ^ N
        tiles = Rotate(tiles); // < N
        tiles = Rotate(tiles); // v N
        tiles = Tilt(tiles);   // tilt E

        tiles = Rotate(tiles); // > N
        tiles = Rotate(tiles); // ^ N

        return tiles;
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
