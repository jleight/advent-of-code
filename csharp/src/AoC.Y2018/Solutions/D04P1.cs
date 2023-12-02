using System.Text.RegularExpressions;

namespace AoC.Y2018.Solutions;

public class D04P1 : ISolution
{
    private static readonly Regex _parser = new (@"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)");
    private static readonly Regex _guard = new (@"Guard #(\d+) begins shift");

    public Task Run(SolutionContext context)
    {
        var logs = context.InputLines
            .Select(Parse)
            .OrderBy(l => l.Year)
            .ThenBy(l => l.Month)
            .ThenBy(l => l.Day)
            .ThenBy(l => l.Hour)
            .ThenBy(l => l.Minute)
            .ToList();

        var heatmaps = new Dictionary<string, int[]>();

        var guard = string.Empty;
        var start = 0;
        foreach (var log in logs)
        {
            if (log.Message == "falls asleep")
            {
                start = log.Minute;
            }
            else if (log.Message == "wakes up")
            {
                if (!heatmaps.ContainsKey(guard))
                    heatmaps[guard] = new int[60];
                for (var i = start; i < log.Minute; i++)
                    heatmaps[guard][i] += 1;
            }
            else
            {
                guard = _guard.Match(log.Message).Groups[1].Value;
            }
        }

        var laziest = heatmaps
            .Select(p => new
            {
                Guard = Convert.ToInt32(p.Key, 10),
                Sum = p.Value.Sum(),
                Max = p.Value.Max(),
                Minute = Array.IndexOf(p.Value, p.Value.Max())
            })
            .OrderByDescending(x => x.Sum)
            .First();

        Console.WriteLine(laziest.Guard * laziest.Minute);
        return Task.CompletedTask;
    }

    private static (int Year, int Month, int Day, int Hour, int Minute, string Message) Parse(string log)
    {
        var match = _parser.Match(log);
        return (
            Convert.ToInt32(match.Groups[1].Value, 10),
            Convert.ToInt32(match.Groups[2].Value, 10),
            Convert.ToInt32(match.Groups[3].Value, 10),
            Convert.ToInt32(match.Groups[4].Value, 10),
            Convert.ToInt32(match.Groups[5].Value, 10),
            match.Groups[6].Value
        );
    }
}
