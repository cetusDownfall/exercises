package cellularAutomata.langton;
import cellularAutomata.utils.*;
import cellularAutomata.utils.RemDupes;
import javafx.application.Application;
import javafx.stage.Stage;
import javafx.animation.AnimationTimer;
import javafx.animation.Animation;
import javafx.scene.paint.Color;
import javafx.scene.Scene;
import javafx.scene.image.WritableImage;
import javafx.scene.image.*;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.*;
import javafx.animation.Timeline;
import javafx.animation.KeyFrame;
import javafx.scene.text.Text;
import javafx.scene.control.*;
import javafx.scene.layout.*;
import javafx.animation.KeyValue;
import javafx.animation.PauseTransition;
import javafx.animation.SequentialTransition;
import javafx.util.Duration;
import javafx.event.EventHandler;
import javafx.beans.value.ObservableValue;
import javafx.beans.value.ChangeListener;
import javafx.beans.property.*;
import javafx.beans.binding.*;
import javafx.beans.value.*;
import javafx.stage.*;
import javafx.event.ActionEvent;
import javax.imageio.ImageIO;
import java.util.Random;
import java.util.ArrayList;
import java.util.*;

public class AntAnim extends Application{
	public static void main(String[] args){
		launch(args);
	}
	@Override
	public void start(Stage primaryStage)throws Exception{
		Text antCtr = new Text("");
		Button startBtn = new Button("Start Animation");
		startBtn.setDefaultButton(true);
		Button endBtn = new Button("End Animation");
		endBtn.setCancelButton(true);
		ToggleButton toroid = new ToggleButton("Toroid field");
		TextField rgb1 = new TextField("FFFFFF");
		rgb1.setPromptText("Field");
		TextField rgb2 = new TextField("000000");
		rgb2.setPromptText("Trail");
		TextField antNum = new TextField("1");
		antNum.setPromptText("Ant Qt");
		TextField spf = new TextField("17");
		spf.setPromptText("Ms/F");
		RadioButton rand = new RadioButton("Random spawn");
		RadioButton center = new RadioButton("Centered spawn");
		center.setSelected(true);
		ToggleGroup patt = new ToggleGroup();
		rand.setToggleGroup(patt);
		center.setToggleGroup(patt);
		Slider dimSld = new Slider(0, 512, 64);
		Text currentSliderPosition = new Text(Double.toString(dimSld.getValue()));
		dimSld.valueProperty().addListener(new ChangeListener<Number>(){
			@Override
			public void changed(ObservableValue<?extends Number> arg0, Number arg1, Number arg2){
				currentSliderPosition.setText(arg2.toString());
			}
		});
		dimSld.setMajorTickUnit(128);
		dimSld.setMinorTickCount(7);
		dimSld.setSnapToTicks(true);
		dimSld.setShowTickMarks(true);
		dimSld.setShowTickLabels(true);
		dimSld.setBlockIncrement(1);
		BorderPane borderPane = new BorderPane();
		VBox fInputs = new VBox();
		fInputs.getChildren().addAll(rgb1,rgb2,antNum,spf,toroid,rand,center,new VBox(new Label("Side length(16-512)"), dimSld), currentSliderPosition);
		borderPane.setLeft(fInputs);
		HBox iInputs = new HBox();
		iInputs.getChildren().addAll(startBtn, endBtn, new Label("Unique Ants Alive: "), antCtr);
		borderPane.setBottom(iInputs);

		Timeline timeline = new Timeline();
		timeline.setCycleCount(Timeline.INDEFINITE);
		final long timestart = System.currentTimeMillis();
		Group root = new Group();	
		borderPane.setCenter(root);
		ImageView iv = new ImageView();
		iv.setPreserveRatio(true);
		root.getChildren().add(iv);
		Scene scene = new Scene(borderPane, 1024, 860, Color.BLACK);
		primaryStage.setTitle("Langton's Ant Demonstration by Cetus");
		primaryStage.setScene(scene);
		primaryStage.show();
		startBtn.setOnAction(new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent e) {
				RangeFinder check = new RangeFinder(new Point((int)dimSld.getValue(),(int)dimSld.getValue()),new Point(0,0));
				WritableImage graph = new WritableImage((int)dimSld.getValue(),(int)dimSld.getValue());
				PixelWriter pen = graph.getPixelWriter();
				PixelReader read = graph.getPixelReader();
				iv.setImage(graph);
				int dims;
				boolean torus;
				int antCt;
				int pattern = 0;
				Color[] colors = new Color[2];
				ArrayList<Ant> ants = new ArrayList<Ant>();
				ArrayList<Ant> nAnts = new ArrayList<Ant>();
				Random gen = new Random();
				colors[0] = Color.web(rgb1.getText());
				colors[1] = Color.web(rgb2.getText());
				dims = (int) dimSld.getValue();
				currentSliderPosition.setText(new Integer(dims).toString());
				antCt = Integer.parseInt(antNum.getText());
				torus = toroid.isSelected();
				if (rand.isSelected()){
					pattern = 1;
				}else if (center.isSelected()){
					pattern = 0;
				}
				for (int i = 0; i < antCt; i++){
					switch (pattern){
						case 0: ants.add(new Ant((dims/2)+(dims%2), (dims/2)+(dims%2), i%4));break;
						case 1: ants.add(new Ant((int)(dims*gen.nextFloat()), (int)(dims*gen.nextFloat()), (int)(4*gen.nextFloat())));break;
					}
				}
				for (int i = 0; i < graph.getHeight(); i++){
					for (int j = 0; j < graph.getWidth(); j++){
						pen.setColor(i, j, colors[0]);
					}
				}
				KeyFrame kf = new KeyFrame(
					Duration.millis(Double.parseDouble(spf.getText().toString())),
					new EventHandler<ActionEvent>()
					{
						public void handle(ActionEvent ae){
							if (ants.size() > 0){
								Image currG = iv.getImage();
								PixelReader currV = currG.getPixelReader();
								WritableImage nGraph = new WritableImage(currV, dims, dims); 
								PixelWriter nPen = nGraph.getPixelWriter();
								for (Ant a : ants){
									int	r = a.getPos(1);
									int	c = a.getPos(0);
									boolean	isFill = currV.getColor(c,r).equals(colors[1]);
									if (check.contains(a)){
										nPen.setColor(c, r, colors[isFill?0:1]);
									}
									if (torus){
										a.toroidalMove(isFill, dims); 
										nAnts.add(a);
									}else{
										a.move(isFill);
										if (check.contains(a)){
											nAnts.add(a);
										}
									}
								}
								iv.setImage(nGraph);
								iv.setFitWidth(720);
								iv.setFitHeight(720);
								ants.clear();
								ants.addAll(RemDupes.remAnts(nAnts));
								antCtr.setText(new Integer(ants.size()).toString());
								nAnts.clear();
							}else{
								timeline.pause();
							}
						}
					});
				timeline.getKeyFrames().add(kf);
				timeline.playFromStart();
			}});
		endBtn.setOnAction(new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent e){
				timeline.stop();
				timeline.getKeyFrames().clear();
				iv.setImage(new WritableImage(1, 1));
			}
		});
	}
}
