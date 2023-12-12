namespace AoC.Y2023.Solutions;

public class D08P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var maps = context.InputLines[2..]
            .ToDictionary(
                l => l[..3],
                l => (Left: l[7..10], Right: l[12..15]));

        var locations = maps.Keys
            .Where(k => k[2] == 'A')
            .Select(k => new Pair { Location = k, Moves = 0L })
            .ToArray();

        foreach (var pair in locations)
        {
            using var moves = context.InputLines[0]
                .ToArray()
                .Repeat()
                .GetEnumerator();

            while (pair.Location[2] != 'Z')
            {
                moves.MoveNext();

                pair.Location = moves.Current == 'L'
                    ? maps[pair.Location].Left
                    : maps[pair.Location].Right;
                pair.Moves += 1;
            }
        }

        var lcm = locations
            .Select(l => l.Moves)
            .Aggregate(1L, Maths.Lcm);

        Console.WriteLine(lcm);
        return Task.CompletedTask;
    }

    private class Pair
    {
        public string Location { get; set; } = default!;
        public long Moves { get; set; }
    }
}
