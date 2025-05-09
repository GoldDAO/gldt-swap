import { Link } from "react-router-dom";
import NavbarHome from "@components/shared/navbars/Home";
import { Button, Skeleton } from "@components/ui";
import { FrequentlyAskedQuestions } from "@components/landing-page";
import { useLedgerMetadata } from "@hooks/ledger";
import { useGLDNFTLocked } from "@hooks/gld_nft/useGLDNFTLocked";

const Logo = ({ name, alt = "" }: { name: string; alt?: string }) => {
  return (
    <img
      className="flex-none"
      src={`/landing-page-assets/powered-by-logos/${name}.svg`}
      alt={alt}
    />
  );
};

export const LandingPage = () => {
  const logos = [
    { name: "METALOR", alt: "Metalor brand logo" },
    { name: "ORIGYN", alt: "Origyn brand logo" },
    { name: "KPMG", alt: "KPMG brand logo" },
    { name: "LOOMIS", alt: "Loomis brand logo" },
    { name: "ICP", alt: "ICP brand logo" },
    { name: "BITY", alt: "BITY brand logo" },
  ];

  const { data: GLDTMetadata, isSuccess: isSuccessGLDTMetadata } =
    useLedgerMetadata({ ledger: "GLDT" });

  const { data: NFTLocked, isSuccess: isSuccessNFTLocked } = useGLDNFTLocked();

  return (
    <>
      <div className="bg-surface-2 bg-cover-img bg-cover bg-fixed">
        <NavbarHome />
        <section className="container mx-auto px-4 mt-8 xl:mt-0">
          <div className="grid grid-cols-1 xl:grid-cols-2 justify-center items-center">
            <div className="flex justify-center px-4 xl:px-8 pb-16 xl:py-32 order-last xl:order-first">
              <div className="max-w-[800px]">
                <video autoPlay loop muted preload="auto" playsInline>
                  <source
                    src="https://daolink-gold-dao-website-medias.sos-ch-gva-2.exo.io/GLDNFT2GLDT.webm#t=2.106585"
                    type="video/webm"
                  />
                  Your browser does not support the video tag.
                </video>
              </div>
            </div>
            <div className="flex justify-center px-4 xl:px-8 py-4 xl:py-24 text-center xl:text-left">
              <div>
                <div className="text-4xl sm:text-6xl font-bold text-gold/80 mb-2 xl:mb-4">
                  GLDT
                </div>
                <div className="text-2xl xl:text-6xl xl:max-w-[600px]">
                  The future of owning physical gold
                </div>
                <div className="mt-4 mb-8 sm:my-8">
                  <Link to="/swap" target="_blank" rel="noopener noreferrer">
                    <Button className="rounded-xl px-4 xl:px-6 xl:py-4 xl:text-lg">
                      Start swapping
                    </Button>
                  </Link>
                </div>
                <div className="flex flex-col xl:flex-row gap-4">
                  <div className="flex flex-col items-center border border-gold/60 rounded-full px-8 py-2">
                    <div className="text-sm">Total Gold locked in kg</div>
                    <div className="font-semibold">
                      {isSuccessNFTLocked ? (
                        <div>{NFTLocked} kg</div>
                      ) : (
                        <Skeleton className="w-32" />
                      )}
                    </div>
                  </div>
                  <div className="flex flex-col items-center border border-gold/60 rounded-full px-8 py-2">
                    <div className="text-sm">GLDT marketcap in USD</div>
                    <div className="font-semibold">
                      {isSuccessGLDTMetadata ? (
                        <div>{GLDTMetadata.marketCap} $</div>
                      ) : (
                        <Skeleton className="w-32" />
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </div>
      <div className="container mx-auto">
        <section className="border-y border-gold/60">
          <div className="grid grid-cols-1 xl:grid-cols-2">
            <div className="flex justify-center px-8 xl:px-24 pt-16 pb-0 xl:py-24 text-center xl:text-left xl:border-r border-gold/60">
              <div className="text-xl">
                A token backed 100% in perpetuity by{" "}
                <span className="font-semibold">physical gold</span>
              </div>
            </div>
            <div className="flex justify-center px-8 xl:px-24 pt-8 pb-16 xl:py-24 text-center xl:text-left">
              <div className="">
                The GLDT token is a digital asset that combines the stability of
                gold with the liquidity of digital currency. With GDLT, you can
                store your wealth in gold while using it for everyday purchases
                – whether its buying coffee or making larger transactions –
                seamlessly bridging the gap between gold and digital payments.
              </div>
            </div>
          </div>
        </section>
        <section className="border-b border-gold/60 p-16">
          <div className="text-center text-lg font-semibold text-content/60 mb-6">
            POWERED BY
          </div>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-6">
            {logos.map(({ name, alt }) => (
              <div className="flex justify-center items-center p-6" key={name}>
                <Logo name={name} alt={alt} />
              </div>
            ))}
          </div>
        </section>
        <section className="border-b border-gold/60">
          <div className="flex flex-col justify-center items-center px-8 xl:px-24 pt-16 pb-16 xl:py-24 text-center">
            <div className="text-2xl">
              Our <span className="font-semibold">technology</span>
            </div>
            <div className="mt-8">
              GLDTs and their underlying smart contracts run entirely on the ICP
              blockchain.
              <br />
              In the future, GLDTs will become cross-platform and multi-chain.
              <br />
              This heralds a new era in which physical gold can be transferred
              using blockchain technology.
              <br />
              To learn more about how GLDT and swapping GLD NFTs works, please
              refer to the FAQ or review the whitepaper.
            </div>
          </div>
        </section>
        <section className="border-y border-gold/60">
          <div className="grid grid-cols-1 xl:grid-cols-2">
            <div className="flex flex-col justify-center px-8 xl:px-24 pt-16 pb-0 xl:py-24 text-center xl:text-left xl:border-r border-gold/60">
              <div className="text-2xl">
                Determining the{" "}
                <span className="font-semibold">price of GLDT</span>
              </div>
              <div className="mt-8">
                The price of GLDT is directly correlated with the spot price of
                physical gold.
                <br />
                The market determines the price of gold, which is then used to
                calculate the value of GLDT.
                <br />
                It's important to note that every 100 GLDT equals 1 gram of
                gold.
                <br />
                This system operates 24/7, accessible all around the world with
                lowest fees.
              </div>
            </div>
            <div className="flex justify-center px-4 xl:px-24 pt-8 pb-16 xl:py-24 text-center xl:text-left bg-surface-2 bg-cover-img bg-cover">
              <img
                className="flex-none"
                src={`/landing-page-assets/swap-screenshot.svg`}
              />
            </div>
          </div>
        </section>

        <section className="container mx-auto py-8 xl:py-24 max-w-6xl">
          <div className="flex flex-col items-center px-8">
            <div className="text-4xl sm:text-6xl font-bold text-gold/80 mb-1 xl:mb-2">
              Get started
            </div>
            <div className="text-2xl xl:text-4xl xl:max-w-[600px]">
              with GLD NFTs
            </div>

            <div className="text-lg text-center my-4">
              Explore the future of ownership of physical gold and acquire your
              GLD NFTs today on BITY Gold.
            </div>
            <Link
              to="https://gold.bity.com/"
              target="_blank"
              rel="noopener noreferrer"
            >
              <Button>Buy GLD NFTs</Button>
            </Link>
          </div>

          <div className="grid grid-cols-1 xl:grid-cols-3 mt-16 px-8">
            <div className="mb-8 xl:mb-0 text-center xl:text-left">
              <div className="text-lg">Frequently Asked Questions</div>
              <Link to="/faqs" className="text-gold/80">
                View more FAQs
              </Link>
            </div>
            <FrequentlyAskedQuestions limit={3} className="col-span-2" />
          </div>
        </section>
      </div>
    </>
  );
};
