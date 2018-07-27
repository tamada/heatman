package jp.ac.kyoto_su.tamadalab.heatmapper;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Optional;

import javax.imageio.ImageIO;

public class HeatMapGenerator {
    private static final Color OPAQUE = new Color(255, 0, 0, 0);

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
        Integer scale = args.pixel().orElse(1);
        BufferedImage image = new BufferedImage(scale * (dim.width + 1), scale * (dim.height + 1), BufferedImage.TYPE_INT_ARGB);
        paint(table, image.createGraphics(), dim, scale);

        BufferedImage dest = new BufferedImage(args.width().orElse(image.getWidth()),
                args.height().orElse(image.getHeight()), BufferedImage.TYPE_INT_ARGB);
        dest.getGraphics().drawImage(image, 0, 0, dest.getWidth(), dest.getHeight(), null);
        return dest;
    }

    private void paint(DataTable table, Graphics2D g, Dimension dim, Integer pixelSize) {
        for (int i = 0; i < dim.width; i++) {
            for (int j = 0; j < dim.height; j++) {
                Optional<Double> value = table.get(i, j);
                g.setColor(color(value));
                g.fillRect(i * pixelSize, j * pixelSize, pixelSize, pixelSize);
            }
        }
    }

    private Color color(Optional<Double> value) {
        return value.map(number -> color(number)).orElse(OPAQUE);
    }

    private Color color(Double value) {
        float result = (float)(((1 - value) * 240) / 360);
        return Color.getHSBColor(result, 1f, 1f);
    }
}
