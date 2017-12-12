package cellularAutomata.wolfram;
import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Scene;
import javafx.scene.paint.Color;
import javafx.scene.image.Image;
import javafx.scene.image.WritableImage;
import javafx.scene.image.ImageView;
import javafx.scene.image.PixelReader;
import javafx.scene.image.PixelWriter;
import javafx.scene.Group;
import java.util.ArrayList;
import java.util.List;
import java.util.Arrays;
import java.util.Random;
import java.util.*;
/**Takes two integers as command line arguments.
	First integer is rule number (0-255),
	switch v^
	Second integer is spawn condition mode:
		0: randomly place equal amounts of black and white dots.
		1: a single black dot in the center.
		-1: a single white dot surrounded by black in the center.
		2: all white.
		-2: all black.
		3: use custom spawn pattern. Must have changed the SPAWN_PATTERN byte array within the start method.
	Third integer is the dimensions.
*/
public class Elementary extends Application{
	static int entry;
	static int mode;
	static int dimmer;
	WritableImage ram;
	private class Rule{
		private byte r = 0;
		public Rule(byte n){
			r=n;
		}
		public boolean next(byte i){
			return (r&(byte)Math.pow(2,i))!=0;
		}
	}
	private class Point{
		private Color state = Color.WHITE;
		private PixelReader read;
		private PixelWriter write;
		private int [] pos = new int[2];
		private WritableImage wi;
		public Point(WritableImage imv,int posr, int posc){
			pos[0] = posr;
			pos[1] = posc;
			wi = imv;
			read = wi.getPixelReader();
			write = wi.getPixelWriter();
			setColor(read.getColor(posc,posr));
		}
		public boolean getState(){
			return state.equals(Color.BLACK);
		}
		public Color getColor(){
			return state;
		}
		public void setColor(Color c){
			state = c;
			write.setColor(pos[1],pos[0],state);
		}
		public void toggleState(){
			state = state.equals(Color.WHITE)?Color.BLACK:Color.WHITE;
			setColor(state);
		}
		public int getPos(int i){
			return pos[i>=1?1:0];
		}
		public byte getNeighbors(Point[] last){
			byte out = 0;
			for(int i = -1; i < 2; i++){
				int index = (int)(((getPos(1)+i)+wi.getWidth())%wi.getWidth());
				if(last[index].getState()){
					out+=1;
				}
				out=(byte)(out<<1);
			}
			return (byte)(out>>1);
		}
	}
	public static void main(String[] args){
		entry = Integer.parseInt(args[1]);
		mode = Integer.parseInt(args[0]);
		dimmer = Integer.parseInt(args[2]);
		launch(args);
	}
	public void start(Stage stage){
		final int DIMS = dimmer;
		final Rule WOLF = new Rule((byte)entry);
		final int CENTER = (int)((DIMS/2)+.5);
		WritableImage ram = new WritableImage(DIMS,DIMS);
		PixelReader read = ram.getPixelReader();
		PixelWriter write = ram.getPixelWriter();
		if(mode!=0){
			Color t = mode>0?Color.BLACK:Color.WHITE;
			Color f = mode>0?Color.WHITE:Color.BLACK;
			if(Math.abs(mode)<3){
				for(int i = 0; i < ram.getWidth(); i++){
					write.setColor(i,0,f);
				}
				if(Math.abs(mode)<2){
					write.setColor((int)(ram.getWidth()/2+.5),0,t);
				}
			}
		}else{
			Random gen = new Random();
			List<Integer> tPos = new ArrayList<Integer>();
			for(int i = 0; i < CENTER; i++){
				int atpt = ((int)(gen.nextFloat()*(DIMS-i)+.5));
				if(i==0){
					tPos.add(new Integer(atpt));
				}else{
					for(Integer ig : tPos){
						if(atpt<(ig-tPos.indexOf(ig))){
							tPos.add(tPos.indexOf(ig), new Integer(atpt));
							break;
						}else{
							atpt+=tPos.indexOf(ig)+1;
							tPos.add(tPos.indexOf(ig)+1, new Integer(atpt));
							break;
						}
					}
				}	
			}
			for(Integer ig : tPos){
				write.setColor(ig,0,Color.BLACK);
			}
		}
		for(int i = 0; i < ram.getHeight(); i++){
			Point[] curr = new Point[DIMS];
			Point[] next = new Point[DIMS];
			for(int j = 0; j < ram.getWidth(); j++){
				curr[j]=new Point(ram,i,j);
			}
			for(int j = 0; j < ram.getWidth(); j++){
				if(i!=ram.getHeight()-1){
					next[j]=new Point(ram,i+1,j);
					if(WOLF.next(next[j].getNeighbors(curr))){
						next[j].setColor(Color.BLACK);
					}else{
						next[j].setColor(Color.WHITE);
					}
				}
			}
		}
		ImageView iv = new ImageView();
		iv.setFitWidth(DIMS>720?DIMS:720);
		iv.setFitHeight(DIMS>720?DIMS:720);
		iv.setImage(ram);
		Group root = new Group();
		root.getChildren().add(iv);
		Scene scene = new Scene(root,DIMS>720?DIMS:720,DIMS>720?DIMS:720,Color.WHITE);
		stage.setScene(scene);
		stage.show();
	}
}
