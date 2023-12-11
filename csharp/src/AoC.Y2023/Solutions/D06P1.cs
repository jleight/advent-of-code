namespace AoC.Y2023.Solutions;

public class D06P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var times = context.InputLines[0]
            .Split(':')[1]
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .ToArray();

        var records = context.InputLines[1]
            .Split(':')[1]
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .ToArray();

        var result = 1;

        for (var race = 0; race < times.Length; race++)
        {
            var time = times[race];
            var record = records[race];

            var wins = 0;

            for (var i = 1; i < time; i++)
            {
                var distance = (time - i) * i;
                if (distance > record)
                    wins += 1;
            }

            result *= wins;
        }

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}
