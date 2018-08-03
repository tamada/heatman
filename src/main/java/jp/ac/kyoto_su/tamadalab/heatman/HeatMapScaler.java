package jp.ac.kyoto_su.tamadalab.heatman;

import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.stream.IntStream;

import javax.imageio.ImageIO;

public class HeatMapScaler {
    private BufferedImage image;

    public HeatMapScaler(Arguments arguments) {
        image = createImage(arguments.colorMapper());
    }

    public void store(String dest) throws IOException {
        String format = dest.substring(dest.lastIndexOf('.') + 1, dest.length());
        ImageIO.write(image, format, new File(dest));
    }

    private BufferedImage createImage(ColorMapper mapper) {
        BufferedImage image = new BufferedImage(255, 10, BufferedImage.TYPE_INT_ARGB);
        Graphics2D g = image.createGraphics();
        IntStream.rangeClosed(0, 255)
        .forEach(index -> draw(index, mapper, g));
        return image;
    }

    private void draw(int index, ColorMapper mapper, Graphics2D g) {
        g.setColor(mapper.map(1.0 * index / 255));
        g.drawRect(index, 0, 1, 10);
    }
}
