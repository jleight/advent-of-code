namespace AoC.Y2023.Solutions;

public class D02P1 : ISolution
{
    private readonly HashSet<string> _validColors = new()
    {
        "red",
        "green",
        "blue"
    };

    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(Game.Parse)
            .Where(IsValid)
            .Select(g => g.Id)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;

        bool IsValid(Game game)
        {
            if (game.Rounds.Any(r => r.Cubes.GetValueOrDefault("red") > 12))
                return false;
            if (game.Rounds.Any(r => r.Cubes.GetValueOrDefault("green") > 13))
                return false;
            if (game.Rounds.Any(r => r.Cubes.GetValueOrDefault("blue") > 14))
                return false;
            if (game.Rounds.Any(r => r.Colors.Except(_validColors).Any()))
                return false;
            return true;
        }
    }

    private record Game(
        int Id,
        ICollection<GameRound> Rounds)
    {
        public static Game Parse(string line)
        {
            var split = line.Split(":", StringSplitOptions.TrimEntries);

            var id = split
                .First()
                .Replace("Game ", string.Empty)
                .ConvertTo<int>();

            var rounds = split
                .Last()
                .Split(";", StringSplitOptions.TrimEntries)
                .Select(GameRound.Parse)
                .ToArray();

            return new Game(
                id,
                rounds);
        }
    }

    private record GameRound(
        Dictionary<string, int> Cubes,
        ISet<string> Colors)
    {
        public static GameRound Parse(string line)
        {
            var cubes = line
                .Split(",", StringSplitOptions.TrimEntries)
                .Select(c => c.Split(" ", StringSplitOptions.TrimEntries))
                .ToDictionary(
                    c => c.Last(),
                    c => c.First().ConvertTo<int>());

            var colors = cubes
                .Keys
                .ToHashSet();

            return new GameRound(
                cubes,
                colors);
        }
    }
}
