import javafx.application.Application;
import javafx.scene.Scene;
import javafx.stage.Stage;
import javafx.scene.Group;
import javafx.scene.paint.Color;
import javafx.scene.layout.*;
import javafx.scene.image.Image;
import javafx.scene.image.WritableImage;
import javafx.scene.image.PixelReader;
import javafx.scene.image.PixelWriter;
import javafx.scene.image.ImageView;
import javafx.event.EventHandler;
import javafx.event.*;
import javafx.animation.AnimationTimer;
import javafx.animation.Timeline;
import javafx.animation.KeyFrame;
import javafx.util.Duration;
import java.util.ArrayList;
import java.util.Set;
import java.util.HashSet;
import java.util.HashMap;
import java.util.Map;
public class Dots extends Application{
	private class Cell{
		public Cell(boolean[][] gri
	public static void main(String[] args){launch(args);}
	@Override
	public void start(Stage stage){
		Group group = new Group();
		WritableImage wi = new WritableImage(128,128);
		WritableImage wi2 = new WritableImage(128,128);
		PixelReader pr = wi.getPixelReader();
		PixelReader pr2 = wi2.getPixelReader();
		PixelWriter pw = wi.getPixelWriter();
		PixelWriter pw2 = wi2.getPixelWriter();
		ImageView iv = new ImageView();
		iv.setImage(wi);
		iv.setFitWidth(1024);
		iv.setFitHeight(1024);
		group.getChildren().add(iv);
		Scene scene = new Scene(group, 1024, 1024, Color.BLACK);
		stage.setScene(scene);
		final long begin = System.currentTimeMillis();
		Timeline tmln = new Timeline();
		tmln.setCycleCount(Timeline.INDEFINITE);
		tmln.getKeyFrames().add(new KeyFrame(Duration.millis(17),new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent ae){
				long now = Math.abs(begin-System.currentTimeMillis());
				for(int i=0;i<wi.getHeight();i++){
					for(int j=0;j<wi.getWidth();j++){
						pw.setColor(j,i,Color.rgb((j*(int)now)%255,(i*(int)now)%255,(j*i+(int)now)%255).invert());
					}
				}
			}
		}));

		tmln.play();
		stage.show();
	}
}
