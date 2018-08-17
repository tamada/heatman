package jp.ac.kyoto_su.tamadalab.heatman;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Optional;

import javax.imageio.ImageIO;

import jp.ac.kyoto_su.tamadalab.heatman.entities.DataTable;

public class HeatMapGenerator {
    private BufferedImage image;

    public HeatMapGenerator(String dataFile, Arguments args) {
        DataTable table = new DataTable(dataFile);
        this.image = createImage(table, args);
    }

    public void store(String dest) throws IOException {
        String format = dest.substring(dest.lastIndexOf('.') + 1, dest.length());
        ImageIO.write(image, format, new File(dest));
    }

    private BufferedImage createImage(DataTable table, Arguments args) {
        Dimension dim = table.size();
        int scale = args.pixel();
        BufferedImage image = new BufferedImage(scale * dim.width, scale * dim.height, BufferedImage.TYPE_INT_ARGB);
        Graphics2D g = image.createGraphics();
        paint(table, g, dim, scale, args.colorMapper());
        args.auxiliraryStep().ifPresent(step -> drawAuxiliaryLines(step, g, dim, args));

        return image;
    }

    private void drawAuxiliaryLines(int step, Graphics2D g, Dimension dim, Arguments args) {
        int pixel = args.pixel();
        g.setColor(Color.WHITE);
        for(int i = step; i <= dim.getHeight(); i += step)
            g.drawLine(0, i * pixel, dim.width * pixel, i * pixel);
        for(int j = step; j <= dim.getWidth(); j += step)
            g.drawLine(j * pixel, 0, j * pixel, dim.height * pixel);
    }

    private void paint(DataTable table, Graphics2D g, Dimension dim, Integer pixelSize, ColorMapper mapper) {
        for (int i = 0; i < dim.width; i++) {
            for (int j = 0; j < dim.height; j++) {
                Optional<Double> value = table.get(i, j);
                g.setColor(mapper.map(value));
                g.fillRect(i * pixelSize, j * pixelSize, pixelSize, pixelSize);
            }
        }
    }
}
