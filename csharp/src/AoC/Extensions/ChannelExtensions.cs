using System.Threading.Channels;

namespace AoC.Extensions;

public static class ChannelExtensions
{
    public static async IAsyncEnumerable<(TLeft Left, TRight Right)> Zip<TLeft, TRight>(
        this ChannelReader<TLeft> left,
        ChannelReader<TRight> right)
    {
        while (await left.WaitToReadAsync() && await right.WaitToReadAsync())
            while (left.TryRead(out var l) && right.TryRead(out var r))
                yield return (l, r);
    }

    public static IAsyncEnumerable<(TLeft Left, TRight Right)> Zip<TLeft, TRight>(
        this Channel<TLeft> left,
        Channel<TRight> right)
    {
        return Zip(left.Reader, right.Reader);
    }
}
