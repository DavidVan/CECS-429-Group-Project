
import java.util.Scanner;


public class DiskEngine {

   public static void main(String[] args) {
      Scanner scan = new Scanner(System.in);

      System.out.println("Menu:");
      System.out.println("1) Build index");
      System.out.println("2) Read and query index");
      System.out.println("Choose a selection:");
      int menuChoice = scan.nextInt();
      scan.nextLine();

      switch (menuChoice) {
         case 1:
            System.out.println("Enter the name of a directory to index: ");
            String folder = scan.nextLine();

            IndexWriter writer = new IndexWriter(folder);
            // You must hook up your code to create an inverted index object,
            // then pass it to the WriteIndex method of the writer.

            break;

         case 2:
            System.out.println("Enter the name of an index to read:");
            String indexName = scan.nextLine();

            DiskInvertedIndex index = new DiskInvertedIndex(indexName);

            while (true) {
               System.out.println("Enter one or more search terms, separated " +
                "by spaces:");
               String input = scan.nextLine();

               if (input.equals("EXIT")) {
                  break;
               }

               int[] postingsList = index.GetPostings(input.toLowerCase());

               if (postingsList == null) {
                  System.out.println("Term not found");
               }
               else {
                  System.out.print("Docs: ");
                  for (int post : postingsList) {
                     System.out.println("Doc# " + post);
                  }
                  System.out.println();
                  System.out.println();
               }
            }

            break;
      }
   }
}
