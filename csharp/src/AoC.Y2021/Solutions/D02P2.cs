namespace AoC.Y2021.Solutions;

[UsedImplicitly]
public class D02P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(Parse)
            .ToArray();

        var position = 0;
        var depth = 0;
        var aim = 0;

        foreach (var command in input)
        {
            if (command.DeltaAxis == Axis.Aim)
            {
                aim += command.Delta;
            }
            else
            {
                position += command.Delta;
                depth += aim * command.Delta;
            }
        }

        Console.WriteLine(position * depth);
        return Task.CompletedTask;
    }

    private static (Axis DeltaAxis, int Delta) Parse(string line)
    {
        var split = line.Split(" ");
        var direction = split[0].ToLowerInvariant();
        var delta = int.Parse(split[1]);

        return direction switch
        {
            "forward" => (Axis.Position, delta),
            "up" => (Axis.Aim, -delta),
            "down" => (Axis.Aim, delta),
            _ => throw new InvalidOperationException($"{direction} is an invalid direction!")
        };
    }

    private enum Axis
    {
        Position,
        Aim
    }
}
