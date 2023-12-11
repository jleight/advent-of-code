namespace AoC.Y2023.Solutions;

public class D06P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var timeDigits = context.InputLines[0]
            .Split(':')[1]
            .Replace(" ", string.Empty);
        var time = long.Parse(new string(timeDigits));

        var recordDigits = context.InputLines[1]
            .Split(':')[1]
            .Replace(" ", string.Empty);
        var record = long.Parse(new string(recordDigits));

        var wins = 0L;

        for (var i = 1; i < time; i++)
        {
            var distance = (time - i) * i;
            if (distance > record)
                wins += 1;
        }

        Console.WriteLine(wins);
        return Task.CompletedTask;
    }
}
