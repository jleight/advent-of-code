namespace AoC.Y2023.Solutions;

public class D07P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(l => l.Split(' '))
            .Select(l => new
            {
                Hand = l[0],
                Bid = long.Parse(l[1]),
                Strength = CalculateStrength(l[0])
            })
            .OrderBy(h => h.Strength)
            .ThenBy(h => CalculateStrength(h.Hand[0]))
            .ThenBy(h => CalculateStrength(h.Hand[1]))
            .ThenBy(h => CalculateStrength(h.Hand[2]))
            .ThenBy(h => CalculateStrength(h.Hand[3]))
            .ThenBy(h => CalculateStrength(h.Hand[4]))
            .Select((h, i) => h.Bid * (i + 1))
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static int CalculateStrength(string hand)
    {
        var counts = hand
            .GroupBy(c => c)
            .Select(g => g.Count())
            .OrderByDescending(c => c)
            .ToArray();

        return counts switch
        {
            [5] => 7,
            [4, 1] => 6,
            [3, 2] => 5,
            [3, 1, 1] => 4,
            [2, 2, 1] => 3,
            [2, 1, 1, 1] => 2,
            _ => 1
        };
    }

    private static int CalculateStrength(char card)
    {
        return card switch
        {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card - '0'
        };
    }
}
