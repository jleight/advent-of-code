using System.Threading.Channels;

namespace AoC.Y2023.Solutions;

public class D05P2 : ISolution
{
    public async Task Run(SolutionContext context)
    {
        var seeds = context.InputLines[0]
            .Replace("seeds: ", string.Empty)
            .Split(' ')
            .Select(long.Parse)
            .Chunk(2)
            .SelectMany(c =>
            {
                var s = new long[c[1]];
                for (var i = 0; i < c[1]; i++)
                    s[i] = c[0] + i;
                return s;
            });

        var maps = new Dictionary<string, Map>();

        for (var i = 2; i < context.InputLines.Length; i++)
        {
            var header = context.InputLines[i];
            var types = header.Split(' ')[0].Split('-');
            var source = types[0];
            var destination = types[2];

            var converters = context.InputLines
                .Skip(i + 1)
                .TakeWhile(l => l.Length > 0)
                .Select(Converter.Parse)
                .OrderBy(c => c.SourceStart)
                .ToList();
            i += converters.Count + 1;

            maps[source] = new Map(destination, converters);
        }

        var convert = Channel.CreateUnbounded<long>(
            new UnboundedChannelOptions
            {
                SingleWriter = true,
                SingleReader = false
            });
        var minify = Channel.CreateUnbounded<long>(
            new UnboundedChannelOptions
            {
                SingleWriter = false,
                SingleReader = true
            });

        var pushTask = Push(seeds, convert);
        var minifyTask = Minify(minify);

        var convertTasks = Enumerable
            .Range(0, Environment.ProcessorCount)
            .Select(_ => Convert(convert, minify, maps))
            .ToList();

        await pushTask;
        await Task.WhenAll(convertTasks);

        minify.Writer.Complete();

        Console.WriteLine(await minifyTask);
    }

    private static async Task Push(
        IEnumerable<long> seeds,
        ChannelWriter<long> channel)
    {
        foreach (var seed in seeds)
            await channel.WriteAsync(seed);
        channel.Complete();
    }

    private static async Task Convert(
        ChannelReader<long> seeds,
        ChannelWriter<long> channel,
        IReadOnlyDictionary<string, Map> maps)
    {
        try
        {
            while (true)
            {
                var value = await seeds.ReadAsync();
                var unit = "seed";

                while (unit != "location")
                    (value, unit) = maps[unit].Convert(value);

                await channel.WriteAsync(value);
            }
        }
        catch (ChannelClosedException)
        {
            // done
        }
    }

    private static async Task<long> Minify(
        ChannelReader<long> converted)
    {
        var min = long.MaxValue;

        try
        {
            while (true)
            {
                min = Math.Min(
                    min,
                    await converted.ReadAsync());
            }
        }
        catch (ChannelClosedException)
        {
            // done
        }

        return min;
    }

    private record Map(
        string Destination,
        List<Converter> Converters)
    {
        private readonly long _min = Converters.Min(c => c.SourceStart);
        private readonly long _max = Converters.Max(c => c.SourceEnd);

        public (long, string) Convert(long x)
        {
            if (x < _min || x > _max)
                return (x, Destination);

            foreach (var converter in Converters)
            {
                if (x < converter.SourceStart)
                    break;
                if (x > converter.SourceEnd)
                    continue;
                return (converter.Convert(x), Destination);
            }

            return (x, Destination);
        }
    }

    private record Converter(
        long SourceStart,
        long SourceEnd,
        long DestinationStart)
    {
        public static Converter Parse(string line)
        {
            var numbers = line
                .Split(' ')
                .Select(long.Parse)
                .ToArray();
            return new Converter(
                numbers[1],
                numbers[1] + numbers[2] - 1,
                numbers[0]);
        }

        public long Convert(long x)
            => DestinationStart + (x - SourceStart);
    }
}
