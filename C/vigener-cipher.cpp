#include<bits/stdc++.h>
using namespace std;

int main()
{

	while(1){
	cout<<"Enter selection \n1.Encryption \n2.Decryption \nExit\n";
	int t;
	cin>>t;
	if(t==1)
	{

    string s1,s2;
    cout<<"Text to be encrypted: ";
    cin>>s1;

    cout<<"Key: ";
    cin>>s2;
    string s3=s1;
    for(int i=0;i<s1.size();i++)
    {
      s3[i]=s2[i%s2.size()];
    }
    for(int i=0;i<s1.size();i++)
    {
      s3[i]=((s1[i]+s3[i]-'a'*2)%26+'a');
    }
    cout<<"Encrypted text is: ";
    cout<<s3<<"\n";
  }
  else if(t==2)
  {
    string s1,s2;
    cout<<"Text to be decrypted: ";
    cin>>s1;

    cout<<"Key: ";
    cin>>s2;
    string s3=s1;
    for(int i=0;i<s1.size();i++)
    {
      s3[i]=s2[i%s2.size()];
    }
    cout<<s3;
    for(int i=0;i<s1.size();i++)
    {
      s3[i]=((s1[i]-s3[i]+26)%26+'a');
    }
    cout<<"Decrypted text is: ";
    cout<<s3<<"\n";
  }
  else{
    break;
  }
}
}
