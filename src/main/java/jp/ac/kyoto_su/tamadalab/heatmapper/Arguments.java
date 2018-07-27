package jp.ac.kyoto_su.tamadalab.heatmapper;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.function.BiConsumer;

import org.kohsuke.args4j.Argument;
import org.kohsuke.args4j.CmdLineException;
import org.kohsuke.args4j.CmdLineParser;
import org.kohsuke.args4j.Option;

public class Arguments {
    @Option(name="-o", aliases="--output", metaVar="DEST.FILE", usage="destination image file. Default is 'heatmap.png.'")
    private String output = "heatmap.png";

    @Option(name="-w", aliases="--width", metaVar="WIDTH", usage="specifies width of resultant image.")
    private String width;

    @Option(name="-h", aliases="--height", metaVar="HEIGHT", usage="specifies height of resultant image.")
    private String height;

    @Option(name="-p", aliases="--pixel", metaVar="PIXEL", usage="specifies a pixel size of result image.")
    private String pixel = "1";

    @Option(name="-H", aliases="--help", help=true, usage="print this message.")
    private boolean help;

    @Argument
    private List<String> args = new ArrayList<>();

    private CmdLineParser parser = new CmdLineParser(this);

    public Arguments(String[] argsArray) throws CmdLineException {
        parse(argsArray);
    }

    private void parse(String[] argsArray) throws CmdLineException {
        parser.parseArgument(argsArray);
        if(args.size() == 0)
            throw new CmdLineException(parser, "no argument is given.");
    }

    public void perform(BiConsumer<String, Arguments> action) {
        if(!printHelpIfNeeded())
            args.forEach(arg -> action.accept(arg, this));
    }

    public String destination() {
        return output;
    }

    public Optional<Integer> pixel(){
        return toInteger(pixel);
    }

    public Optional<Integer> height(){
        return toInteger(height);
    }

    public Optional<Integer> width(){
        return toInteger(width);
    }

    private Optional<Integer> toInteger(String number) {
        return Optional.ofNullable(number)
                .map(num -> Integer.valueOf(num));
    }

    private boolean printHelpIfNeeded() {
        if(help)
            printHelp(parser);
        return help;
    }

    private void printHelp(CmdLineParser parser) {
        System.out.println("java -jar heatmapper.jar [OPTIONS] <DATA.CSV>");
        System.out.println("[OPTIONS]");
        parser.printUsage(System.out);
    }
}
