using System;
using System.Windows;
using System.Windows.Controls;

namespace ChromaticityDiagram
{
    public struct CieCanvasTransform
    {
        // Stałe granice poprawnej podkowy CIE 1931
        private const double CIE_MIN_X = 0.0;
        private const double CIE_MAX_X = 0.8;
        private const double CIE_MIN_Y = 0.0;
        private const double CIE_MAX_Y = 0.9;

        public double MinX, MaxX;
        public double MinY, MaxY;
        public double Scale;
        public double OffsetX, OffsetY;
        public double CanvasWidth, CanvasHeight;

        // MAPPING CIE XY → WPF CANVAS
        public Point Map(double x, double y)
        {
            double px = OffsetX + (x - MinX) * Scale;
            double py = OffsetY + (MaxY - y) * Scale;  // WPF: Y rośnie w dół
            return new Point(px, py);
        }

        // ODWROTNE MAPOWANIE
        public (double x, double y) InverseMap(double px, double py)
        {
            double x = MinX + (px - OffsetX) / Scale;
            double y = MaxY - (py - OffsetY) / Scale;
            return (x, y);
        }

        // ----------- KLUCZOWA POPRAWKA: stałe granice podkowy -----------
        public static CieCanvasTransform ComputeCieCanvasTransform(Canvas canvas)
        {
            double W = canvas.ActualWidth;
            double H = canvas.ActualHeight;

            var t = new CieCanvasTransform();

            if (W <= 0 || H <= 0)
                return t;

            // Ustal granice
            double minX = CIE_MIN_X;
            double maxX = CIE_MAX_X;
            double minY = CIE_MIN_Y;
            double maxY = CIE_MAX_Y;

            double dx = maxX - minX;
            double dy = maxY - minY;

            // Dobierz wspólną skalę
            double scaleX = W / dx;
            double scaleY = H / dy;
            double scale = Math.Min(scaleX, scaleY);

            // Wycentrowanie
            double plottedWidth = dx * scale;
            double plottedHeight = dy * scale;
            double offsetX = (W - plottedWidth) / 2.0;
            double offsetY = (H - plottedHeight) / 2.0;

            t.MinX = minX;
            t.MaxX = maxX;
            t.MinY = minY;
            t.MaxY = maxY;
            t.Scale = scale;
            t.OffsetX = offsetX;
            t.OffsetY = offsetY;
            t.CanvasWidth = W;
            t.CanvasHeight = H;

            return t;
        }
    }
}
