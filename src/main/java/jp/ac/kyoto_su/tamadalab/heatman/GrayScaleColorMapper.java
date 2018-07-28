package jp.ac.kyoto_su.tamadalab.heatman;

import java.awt.Color;

public class GrayScaleColorMapper implements ColorMapper {
    @Override
    public Color map(double value) {
        int color = (int)((1 - value) * 255);
        return new Color(color, color, color);
    }
}
