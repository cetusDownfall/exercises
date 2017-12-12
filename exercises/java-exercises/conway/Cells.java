package cellularAutomata.conway;
import cellularAutomata.utils.*;
import cellularAutomata.utils.IState;
import cellularAutomata.utils.AbstractCellularAnimation;
import javafx.scene.paint.Color;
import javafx.scene.image.WritableImage;
import javafx.scene.image.PixelReader;
import javafx.scene.image.PixelWriter;
import java.util.Random;
import java.util.ArrayList;
import java.util.*;

public class Cells{
	private Cell[][] cellMap;
	private int size;
	private int boxFrac = 4;
	public Cells(int s, int v){
		Random gen = new Random();
		size = s;
		cellMap = new Cell[size][size];
		for (int i = 0; i < size; i++){
			for (int j = 0; j < size; j++){
				cellMap[i][j] = new Cell(i,j,size,gen.nextInt(100)<v);
				cellMap[i][j].addTo(this);
			}
		}
	}
	public Cells(int s){
		size = s;
		RangeFinder filled = new RangeFinder(new Point(size/2-size/boxFrac,size/2-size/boxFrac),new Point(size/2+size/boxFrac,size/2+size/boxFrac));
		cellMap = new Cell[size][size];
		for (int i = 0; i < size; i++){
			for (int j = 0; j < size; j++){
				cellMap[i][j] = new Cell(i,j,size,filled.contains(new Point(i,j)));
				cellMap[i][j].addTo(this);
			}
		}
	}
	public Cells(WritableImage g){
		cellMap = new Cell[(int)g.getHeight()][(int)g.getWidth()];
		size = cellMap.length;
		PixelReader read = g.getPixelReader();
		for (int i = 0; i < size; i++){
			for (int j = 0; j < size; j++){
				cellMap[i][j] = new Cell(i,j,size,read.getColor(j,i).equals(Color.CYAN));
				cellMap[i][j].addTo(this);
			}
		}
	}
	public Cell[][] getCellMap(){
		return cellMap;
	}
	public WritableImage getNextImage(){
		WritableImage g = new WritableImage(size, size);
		PixelWriter pen = g.getPixelWriter();
		for (int i = 0; i < size; i++){
			for (int j = 0; j < size; j++){
				pen.setColor(j,i,cellMap[i][j].getNextState());
			}
		}
		return g;
	}
	public WritableImage getCurrentImage(){
		WritableImage g = new WritableImage(size, size);
		PixelWriter pen = g.getPixelWriter();
		for (int i = 0; i < size; i++){
			for (int j = 0; j < size; j++){
				pen.setColor(j,i,cellMap[i][j].getCurrentState());
			}
		}
		return g;
	}
}
