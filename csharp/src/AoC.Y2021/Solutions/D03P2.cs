namespace AoC.Y2021.Solutions;

[UsedImplicitly]
public class D03P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(l => l.ToCharArray())
            .ToArray();

        var oxygenRatings = input;
        var co2Ratings = input;

        var index = 0;
        while (oxygenRatings.Length > 1)
        {
            oxygenRatings = oxygenRatings
                .GroupBy(l => l[index])
                .OrderByDescending(l => l.Count())
                .ThenBy(l => l.Key == '1' ? 0 : 1)
                .Take(1)
                .SelectMany(g => g)
                .ToArray();
            index += 1;
        }

        index = 0;
        while (co2Ratings.Length > 1)
        {
            co2Ratings = co2Ratings
                .GroupBy(l => l[index])
                .OrderBy(l => l.Count())
                .ThenBy(l => l.Key == '0' ? 0 : 1)
                .Take(1)
                .SelectMany(g => g)
                .ToArray();
            index += 1;
        }

        var oxygenRating = Convert.ToInt32(new string(oxygenRatings[0]), 2);
        var co2Rating = Convert.ToInt32(new string(co2Ratings[0]), 2);

        Console.WriteLine(oxygenRating * co2Rating);
        return Task.CompletedTask;
    }
}
