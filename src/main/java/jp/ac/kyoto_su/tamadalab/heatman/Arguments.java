package jp.ac.kyoto_su.tamadalab.heatman;

import java.util.ArrayList;
import java.util.List;
import java.util.OptionalInt;
import java.util.function.BiConsumer;

import org.kohsuke.args4j.Argument;
import org.kohsuke.args4j.CmdLineException;
import org.kohsuke.args4j.CmdLineParser;
import org.kohsuke.args4j.Option;

public class Arguments {
    @Option(name="-o", aliases="--output", metaVar="DEST.FILE", usage="destination image file.  The format of image is guessed from its extension.")
    private String output = "heatman.png";

    // @Option(name="-w", aliases="--width", metaVar="WIDTH", usage="specifies width of resultant image.")
    // private String width;

    // @Option(name="-h", aliases="--height", metaVar="HEIGHT", usage="specifies height of resultant image.")
    // private String height;

    @Option(name="-p", aliases="--pixel", metaVar="PIXEL", usage="specifies a pixel size of result image.")
    private int pixel = 1;

    @Option(name="-a", aliases="--auxiliary", metaVar="INTERVAL", usage="specifies a interval of auxiliary lines. If this value is less than equal to 0, no auxilirary lines are drawn.")
    private int auxiliary = 0;

    @Option(name="-g", aliases="--gray", usage="output the grayscaled heatmap image.")
    private boolean gray = false;
    
    @Option(name="-s", aliases="--scaler", usage="output the scaler image. If this option is specified, given data files are ignored.")
    private boolean outputScaler = false;

    @Option(name="-H", aliases="--help", usage="print this message.")
    private boolean help = false;

    @Argument
    private List<String> args = new ArrayList<>();

    private CmdLineParser parser = new CmdLineParser(this);

    public Arguments(String[] argsArray) throws CmdLineException {
        parse(argsArray);
    }

    private void parse(String[] argsArray) throws CmdLineException {
        parser.parseArgument(argsArray);
    }

    public void perform(BiConsumer<String, Arguments> action) {
        if(!printHelpIfNeeded())
            args.forEach(arg -> action.accept(arg, this));
    }

    public OptionalInt auxiliraryStep() {
        if(auxiliary <= 0)
            return OptionalInt.empty();
        return OptionalInt.of(auxiliary);
    }

    public boolean isOutputScaler() {
        return !help && outputScaler;
    }

    public String destination() {
        return output;
    }

    public int pixel(){
        return pixel;
    }

//    public Optional<Integer> height(){
//        return toInteger(height);
//    }
//
//    public Optional<Integer> width(){
//        return toInteger(width);
//    }

    public ColorMapper colorMapper() {
        return ColorMapperBuilder.build(gray);
    }

    private boolean helpFlag() {
        return help || (!outputScaler && args.size() == 0);
    }

    private boolean printHelpIfNeeded() {
        if(helpFlag())
            printHelp(parser);
        return help;
    }

    private void printHelp(CmdLineParser parser) {
        System.out.println("java -jar heatmapper.jar [OPTIONS] <DATA.CSV>");
        System.out.println("[OPTIONS]");
        parser.printUsage(System.out);
    }
}
